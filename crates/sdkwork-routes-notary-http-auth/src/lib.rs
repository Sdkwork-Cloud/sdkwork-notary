pub mod context;
pub mod layer;
pub mod test_support;

pub use context::{notary_request_context_from_web, NotaryAuthError, NotaryRequestContext};

pub fn gateway_mount() -> axum::Router {
    axum::Router::new()
}
