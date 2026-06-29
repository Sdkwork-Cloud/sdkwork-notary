pub mod context;
pub mod layer;
pub mod response;
pub mod test_support;

pub use context::{notary_request_context_from_web, NotaryAuthError, NotaryRequestContext};
pub use response::{
    finish_success, success_envelope, success_status_for_notary_app_operation,
    success_status_for_notary_backend_operation, NotaryRouteError,
};

pub fn gateway_mount() -> axum::Router {
    axum::Router::new()
}
