pub mod context;
pub mod envelope;
pub mod layer;
mod pagination;
pub mod response;
pub mod test_support;

pub use context::{notary_request_context_from_web, NotaryAuthError, NotaryRequestContext};
pub use envelope::{envelope_success_data, is_delete_no_content_operation, is_list_operation};
pub use pagination::validate_list_query;
pub use response::{
    finish_success, finish_success_no_content, success_envelope,
    success_status_for_notary_app_operation, success_status_for_notary_backend_operation,
    NotaryRouteError,
};

pub fn gateway_mount() -> axum::Router {
    axum::Router::new()
}
