use std::sync::Arc;

use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{
    handlers,
    manifest::notary_app_api_http_route_manifest,
    service_port::{NotaryAppApiServicePort, NotaryAppApiState},
};
use sdkwork_routes_notary_http_auth::layer::with_dual_token_request_context;

pub fn build_sdkwork_notary_app_api_router(service: Arc<dyn NotaryAppApiServicePort>) -> Router {
    let router = Router::new()
        .route("/app/v3/api/notary/access", get(handlers::retrieve_access))
        .route(
            "/app/v3/api/notary/dashboard/statistics",
            get(handlers::retrieve_dashboard_statistics),
        )
        .route("/app/v3/api/notary/matters", get(handlers::list_matters))
        .route(
            "/app/v3/api/notary/reports/monthly",
            get(handlers::retrieve_monthly_report),
        )
        .route("/app/v3/api/notary/staff", get(handlers::list_staff))
        .route(
            "/app/v3/api/notary/cases",
            get(handlers::list_cases).post(handlers::create_case),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id",
            get(handlers::retrieve_case).patch(handlers::update_case),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/acceptances",
            post(handlers::accept_case),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/rejections",
            post(handlers::reject_case),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/completions",
            post(handlers::complete_case),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/assignments",
            post(handlers::create_assignment),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/parties",
            get(handlers::list_parties).post(handlers::create_party),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/parties/:party_id",
            patch(handlers::update_party).delete(handlers::delete_party),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/parties/:party_id/signatures",
            post(handlers::attach_party_signature),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/parties/:party_id/video_invites",
            post(handlers::create_party_video_invite),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/parties/:party_id/signature_invites",
            post(handlers::create_party_signature_invite),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/files",
            get(handlers::list_case_files).post(handlers::create_case_file),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/download_packages",
            post(handlers::create_download_package),
        )
        .route(
            "/app/v3/api/notary/cases/:case_id/events",
            get(handlers::list_case_events),
        )
        .with_state(NotaryAppApiState::new(service));
    with_dual_token_request_context(router, notary_app_api_http_route_manifest())
}
