use std::sync::Arc;

use axum::Router;
use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};
use sdkwork_notary_case_repository_sqlx::{
    bootstrap_notary_database, connect_notary_database_pool_from_env, PostgresNotaryCaseRepository,
    SqliteNotaryCaseRepository,
};
use sdkwork_routes_notary_app_api::{NotaryAppApiServicePort, NotaryAppRuntimeService};
use sdkwork_routes_notary_backend_api::{NotaryBackendApiServicePort, NotaryBackendRuntimeService};

use crate::adapters::{CommerceOrderPort, DriveWorkspacePort, IamSqlxAppbasePort};

pub struct EmbeddedNotaryAssembly {
    pub router: Router,
}

pub async fn assemble_embedded_notary_application_router_from_env(
) -> Result<EmbeddedNotaryAssembly, String> {
    let _ = dotenvy::dotenv();
    let notary_pool = connect_notary_database_pool_from_env()
        .await
        .map_err(|error| error.to_string())?;
    let _notary_host = bootstrap_notary_database(notary_pool.clone()).await?;
    let iam_pool = connect_service_database_pool_from_env("IAM").await?;
    let commerce_pool = connect_service_database_pool_from_env("COMMERCE").await?;
    let drive_pool = connect_service_database_pool_from_env("DRIVE").await?;
    let runtime = resolve_embedded_notary_runtime_config();

    assemble_embedded_notary_application_router(
        notary_pool,
        iam_pool,
        commerce_pool,
        drive_pool,
        runtime,
    )
    .await
}

pub async fn assemble_embedded_notary_application_router(
    notary_pool: DatabasePool,
    iam_pool: DatabasePool,
    commerce_pool: DatabasePool,
    drive_pool: DatabasePool,
    runtime: EmbeddedNotaryRuntimeConfig,
) -> Result<EmbeddedNotaryAssembly, String> {
    let app_router = match notary_pool {
        DatabasePool::Sqlite(pool, _) => {
            let repository = SqliteNotaryCaseRepository::new(
                pool,
                runtime.tenant_id.clone(),
                runtime.operator_user_id.clone(),
            );
            build_notary_router(
                iam_pool,
                commerce_pool,
                drive_pool,
                runtime,
                repository.clone(),
                repository,
            )
            .await?
        }
        DatabasePool::Postgres(pool, _) => {
            let repository = PostgresNotaryCaseRepository::new(
                pool,
                runtime.tenant_id.clone(),
                runtime.operator_user_id.clone(),
            );
            build_notary_router(
                iam_pool,
                commerce_pool,
                drive_pool,
                runtime,
                repository.clone(),
                repository,
            )
            .await?
        }
    };

    Ok(EmbeddedNotaryAssembly { router: app_router })
}

async fn build_notary_router<Repository>(
    iam_pool: DatabasePool,
    commerce_pool: DatabasePool,
    drive_pool: DatabasePool,
    runtime: EmbeddedNotaryRuntimeConfig,
    app_repository: Repository,
    backend_repository: Repository,
) -> Result<Router, String>
where
    Repository: sdkwork_notary_case_service::NotaryCaseRepositoryPort
        + Clone
        + Send
        + Sync
        + 'static,
{
    let app_drive = DriveWorkspacePort::new(
        drive_pool.clone(),
        runtime.tenant_id.as_str(),
        runtime.operator_user_id.as_str(),
    )
    .await
    .map_err(|error| error.message().to_string())?;
    let backend_drive = DriveWorkspacePort::new(
        drive_pool,
        runtime.tenant_id.as_str(),
        runtime.operator_user_id.as_str(),
    )
    .await
    .map_err(|error| error.message().to_string())?;

    let app_service: Arc<dyn NotaryAppApiServicePort> = Arc::new(NotaryAppRuntimeService::new(
        IamSqlxAppbasePort::new(iam_pool.clone(), runtime.development_mode),
        CommerceOrderPort::new(commerce_pool.clone()),
        app_drive,
        app_repository,
    ));
    let backend_service: Arc<dyn NotaryBackendApiServicePort> = Arc::new(
        NotaryBackendRuntimeService::new(
            IamSqlxAppbasePort::new(iam_pool, runtime.development_mode),
            CommerceOrderPort::new(commerce_pool),
            backend_drive,
            backend_repository,
        ),
    );

    Ok(sdkwork_routes_notary_app_api::gateway_mount(app_service)
        .merge(sdkwork_routes_notary_backend_api::gateway_mount(backend_service)))
}

#[derive(Clone, Debug)]
pub struct EmbeddedNotaryRuntimeConfig {
    pub tenant_id: String,
    pub operator_user_id: String,
    pub development_mode: bool,
}

fn resolve_embedded_notary_runtime_config() -> EmbeddedNotaryRuntimeConfig {
    let environment = std::env::var("SDKWORK_NOTARY_ENVIRONMENT")
        .or_else(|_| std::env::var("SDKWORK_IM_ENVIRONMENT"))
        .unwrap_or_else(|_| "development".to_owned());
    let development_mode = !environment.eq_ignore_ascii_case("production");
    let tenant_id = std::env::var("SDKWORK_NOTARY_TENANT_ID")
        .or_else(|_| std::env::var("SDKWORK_KNOWLEDGEBASE_TENANT_ID"))
        .unwrap_or_else(|_| "100001".to_owned());
    let operator_user_id = std::env::var("SDKWORK_NOTARY_OPERATOR_ID")
        .or_else(|_| std::env::var("SDKWORK_KNOWLEDGEBASE_OPERATOR_ID"))
        .unwrap_or_else(|_| "system".to_owned());

    EmbeddedNotaryRuntimeConfig {
        tenant_id,
        operator_user_id,
        development_mode,
    }
}

async fn connect_service_database_pool_from_env(service: &str) -> Result<DatabasePool, String> {
    let config = DatabaseConfig::from_env(service)
        .map_err(|error| format!("read {service} database config failed: {error}"))?;
    create_pool_from_config(config)
        .await
        .map_err(|error| format!("create {service} database pool failed: {error}"))
}
