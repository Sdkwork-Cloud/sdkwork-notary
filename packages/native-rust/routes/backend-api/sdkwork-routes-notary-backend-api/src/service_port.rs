use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryRequestContext {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub user_id: String,
    pub membership_id: Option<String>,
    pub session_id: String,
    pub app_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryRouteError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

#[async_trait]
pub trait NotaryBackendApiServicePort: Send + Sync {
    async fn handle(
        &self,
        context: NotaryRequestContext,
        operation_id: &'static str,
        path_params: BTreeMap<String, String>,
        body: Value,
    ) -> Result<Value, NotaryRouteError>;
}

#[derive(Clone)]
pub struct NotaryBackendApiState {
    service: Arc<dyn NotaryBackendApiServicePort>,
    default_context: NotaryRequestContext,
}

impl NotaryBackendApiState {
    pub fn new(
        service: Arc<dyn NotaryBackendApiServicePort>,
        default_context: NotaryRequestContext,
    ) -> Self {
        Self {
            service,
            default_context,
        }
    }

    pub fn service(&self) -> Arc<dyn NotaryBackendApiServicePort> {
        Arc::clone(&self.service)
    }

    pub fn request_context(&self) -> NotaryRequestContext {
        self.default_context.clone()
    }
}

impl IntoResponse for NotaryRouteError {
    fn into_response(self) -> Response {
        let body = json!({
            "type": "about:blank",
            "title": self.message,
            "status": self.status.as_u16(),
            "code": self.code,
        });
        (self.status, Json(body)).into_response()
    }
}
