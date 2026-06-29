use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use serde_json::Value;

pub use sdkwork_routes_notary_http_auth::{
    NotaryAuthError, NotaryRequestContext, NotaryRouteError,
};

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
}

impl NotaryBackendApiState {
    pub fn new(service: Arc<dyn NotaryBackendApiServicePort>) -> Self {
        Self { service }
    }

    pub fn service(&self) -> Arc<dyn NotaryBackendApiServicePort> {
        Arc::clone(&self.service)
    }
}
