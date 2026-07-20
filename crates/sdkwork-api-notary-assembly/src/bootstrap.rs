//! Gateway bootstrap for sdkwork-notary.
//! Multi-surface merges mount shared infrastructure routes once at the assembly layer
//! so `/healthz`, `/livez`, `/readyz`, and `/metrics` are not duplicated per surface.

use axum::Router;

pub struct ApiAssembly {
    pub router: Router,
    _embedded_notary: sdkwork_notary_embedded_bootstrap::EmbeddedNotaryAssembly,
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    let embedded =
        sdkwork_notary_embedded_bootstrap::assemble_embedded_notary_application_router_from_env()
            .await?;
    let router = embedded
        .router
        .clone()
        .merge(sdkwork_routes_notary_http_auth::gateway_mount());
    Ok(ApiAssembly {
        router,
        _embedded_notary: embedded,
    })
}
