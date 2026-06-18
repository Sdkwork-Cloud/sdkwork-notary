use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

pub use sdkwork_router_notary_http_auth::{NotaryAuthError, NotaryRequestContext};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryRouteError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl From<NotaryAuthError> for NotaryRouteError {
    fn from(error: NotaryAuthError) -> Self {
        Self {
            status: error.status,
            code: error.code,
            message: error.message,
        }
    }
}

#[async_trait]
pub trait NotaryAppApiServicePort: Send + Sync {
    async fn handle(
        &self,
        context: NotaryRequestContext,
        operation_id: &'static str,
        path_params: BTreeMap<String, String>,
        body: Value,
    ) -> Result<Value, NotaryRouteError>;
}

#[derive(Clone)]
pub struct NotaryAppApiState {
    service: Arc<dyn NotaryAppApiServicePort>,
}

impl NotaryAppApiState {
    pub fn new(service: Arc<dyn NotaryAppApiServicePort>) -> Self {
        Self { service }
    }

    pub fn service(&self) -> Arc<dyn NotaryAppApiServicePort> {
        Arc::clone(&self.service)
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
