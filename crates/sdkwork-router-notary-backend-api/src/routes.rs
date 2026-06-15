use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handlers,
    service_port::{NotaryBackendApiServicePort, NotaryBackendApiState, NotaryRequestContext},
};

pub fn build_sdkwork_notary_backend_api_router(
    service: Arc<dyn NotaryBackendApiServicePort>,
    default_context: NotaryRequestContext,
) -> Router {
    Router::new()
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
        .with_state(NotaryBackendApiState::new(service, default_context))
}
