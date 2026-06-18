use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handlers,
    manifest::notary_backend_api_http_route_manifest,
    service_port::{NotaryBackendApiServicePort, NotaryBackendApiState},
};
use sdkwork_router_notary_http_auth::layer::with_dual_token_request_context;

pub fn build_sdkwork_notary_backend_api_router(
    service: Arc<dyn NotaryBackendApiServicePort>,
) -> Router {
    let router = Router::new()
        .route(
            "/backend/v3/api/notary/organization_profiles",
            get(handlers::list_organization_profiles).post(handlers::create_organization_profile),
        )
        .route(
            "/backend/v3/api/notary/organization_profiles/:organization_profile_id",
            get(handlers::retrieve_organization_profile)
                .patch(handlers::update_organization_profile),
        )
        .route(
            "/backend/v3/api/notary/matters",
            get(handlers::list_matters).post(handlers::create_matter),
        )
        .route(
            "/backend/v3/api/notary/matters/:sku_id",
            patch(handlers::update_matter),
        )
        .route("/backend/v3/api/notary/cases", get(handlers::list_cases))
        .route(
            "/backend/v3/api/notary/cases/:case_id",
            get(handlers::retrieve_case),
        )
        .route(
            "/backend/v3/api/notary/cases/:case_id/assignments",
            post(handlers::create_assignment),
        )
        .route(
            "/backend/v3/api/notary/cases/:case_id/assignments/:assignment_id",
            delete(handlers::delete_assignment),
        )
        .route("/backend/v3/api/notary/staff", get(handlers::list_staff))
        .route(
            "/backend/v3/api/notary/reports/case_summary",
            get(handlers::retrieve_case_summary),
        )
        .with_state(NotaryBackendApiState::new(service));
    with_dual_token_request_context(router, notary_backend_api_http_route_manifest())
}
