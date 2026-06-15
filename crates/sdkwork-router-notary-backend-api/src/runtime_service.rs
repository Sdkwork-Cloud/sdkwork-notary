use std::collections::BTreeMap;

use async_trait::async_trait;
use axum::http::StatusCode;
use sdkwork_notary_core::{NotaryRuntimeContext, NotaryServiceError};
use sdkwork_notary_runtime::{
    handle_notary_backend_operation, AppbasePort, CommercePort, DrivePort,
    NotaryCaseRepositoryPort, NotaryRuntimePorts,
};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::service_port::{NotaryBackendApiServicePort, NotaryRequestContext, NotaryRouteError};

pub struct NotaryBackendRuntimeService<Appbase, Commerce, Drive, Repository>
where
    Appbase: AppbasePort,
    Commerce: CommercePort,
    Drive: DrivePort,
    Repository: NotaryCaseRepositoryPort,
{
    appbase: Mutex<Appbase>,
    commerce: Mutex<Commerce>,
    drive: Mutex<Drive>,
    repository: Mutex<Repository>,
}

impl<Appbase, Commerce, Drive, Repository>
    NotaryBackendRuntimeService<Appbase, Commerce, Drive, Repository>
where
    Appbase: AppbasePort,
    Commerce: CommercePort,
    Drive: DrivePort,
    Repository: NotaryCaseRepositoryPort,
{
    pub fn new(appbase: Appbase, commerce: Commerce, drive: Drive, repository: Repository) -> Self {
        Self {
            appbase: Mutex::new(appbase),
            commerce: Mutex::new(commerce),
            drive: Mutex::new(drive),
            repository: Mutex::new(repository),
        }
    }
}

#[async_trait]
impl<Appbase, Commerce, Drive, Repository> NotaryBackendApiServicePort
    for NotaryBackendRuntimeService<Appbase, Commerce, Drive, Repository>
where
    Appbase: AppbasePort + Send + Sync + 'static,
    Commerce: CommercePort + Send + Sync + 'static,
    Drive: DrivePort + Send + Sync + 'static,
    Repository: NotaryCaseRepositoryPort + Send + Sync + 'static,
{
    async fn handle(
        &self,
        context: NotaryRequestContext,
        operation_id: &'static str,
        path_params: BTreeMap<String, String>,
        body: Value,
    ) -> Result<Value, NotaryRouteError> {
        let runtime_context = runtime_context_from_route(context);
        let mut appbase = self.appbase.lock().await;
        let mut commerce = self.commerce.lock().await;
        let mut drive = self.drive.lock().await;
        let mut repository = self.repository.lock().await;

        handle_notary_backend_operation(
            &runtime_context,
            operation_id,
            path_params,
            body,
            &mut NotaryRuntimePorts {
                appbase: &mut *appbase,
                commerce: &mut *commerce,
                drive: &mut *drive,
                repository: &mut *repository,
            },
        )
        .await
        .map_err(route_error_from_runtime)
    }
}

fn runtime_context_from_route(context: NotaryRequestContext) -> NotaryRuntimeContext {
    NotaryRuntimeContext {
        tenant_id: context.tenant_id,
        organization_id: context.organization_id,
        user_id: context.user_id,
        membership_id: context.membership_id,
        session_id: context.session_id,
        app_id: context.app_id,
    }
}

fn route_error_from_runtime(error: NotaryServiceError) -> NotaryRouteError {
    let status = match error.code() {
        "unauthenticated" => StatusCode::UNAUTHORIZED,
        "unauthorized" => StatusCode::FORBIDDEN,
        "not-found" => StatusCode::NOT_FOUND,
        "conflict" | "invalid-state" => StatusCode::CONFLICT,
        "validation" => StatusCode::BAD_REQUEST,
        "transport" => StatusCode::BAD_GATEWAY,
        "provider-unavailable" => StatusCode::SERVICE_UNAVAILABLE,
        "storage" | "unknown" => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    NotaryRouteError {
        status,
        code: error.code(),
        message: error.message().to_string(),
    }
}
