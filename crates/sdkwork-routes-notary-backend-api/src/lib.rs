pub mod handlers;
pub mod manifest;
pub mod paths;
pub mod routes;
pub mod runtime_service;
pub mod service_port;

pub use manifest::{
    notary_backend_api_http_route_manifest, sdkwork_notary_backend_api_route_manifest,
};
pub use routes::build_sdkwork_notary_backend_api_router;
pub use runtime_service::NotaryBackendRuntimeService;
pub use service_port::{NotaryBackendApiServicePort, NotaryRequestContext, NotaryRouteError};

pub fn gateway_route_manifest() -> HttpRouteManifest {
    notary_backend_api_http_route_manifest()
}

pub fn gateway_mount(service: Arc<dyn NotaryBackendApiServicePort>,) -> Router {
    build_sdkwork_notary_backend_api_router(service)
}
