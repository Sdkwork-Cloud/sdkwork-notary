use std::collections::BTreeMap;

use async_trait::async_trait;
use axum::http::StatusCode;
use sdkwork_notary_case_contract::{NotaryRuntimeContext, NotaryServiceError};
use sdkwork_notary_case_service::{
    handle_notary_app_operation, AppbasePort, CommercePort, DrivePort, NotaryCaseRepositoryPort,
    NotaryRuntimePorts,
};
use serde_json::Value;

use crate::service_port::{NotaryAppApiServicePort, NotaryRequestContext, NotaryRouteError};

pub struct NotaryAppRuntimeService<Appbase, Commerce, Drive, Repository>
where
    Appbase: AppbasePort,
    Commerce: CommercePort,
    Drive: DrivePort,
    Repository: NotaryCaseRepositoryPort,
{
    appbase: Appbase,
    commerce: Commerce,
    drive: Drive,
    repository: Repository,
}

impl<Appbase, Commerce, Drive, Repository>
    NotaryAppRuntimeService<Appbase, Commerce, Drive, Repository>
where
    Appbase: AppbasePort,
    Commerce: CommercePort,
    Drive: DrivePort,
    Repository: NotaryCaseRepositoryPort,
{
    pub fn new(appbase: Appbase, commerce: Commerce, drive: Drive, repository: Repository) -> Self {
        Self {
            appbase,
            commerce,
            drive,
            repository,
        }
    }
}

#[async_trait]
impl<Appbase, Commerce, Drive, Repository> NotaryAppApiServicePort
    for NotaryAppRuntimeService<Appbase, Commerce, Drive, Repository>
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

        handle_notary_app_operation(
            &runtime_context,
            operation_id,
            path_params,
            body,
            &NotaryRuntimePorts {
                appbase: &self.appbase,
                commerce: &self.commerce,
                drive: &self.drive,
                repository: &self.repository,
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
