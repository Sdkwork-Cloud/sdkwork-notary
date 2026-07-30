//! Host-neutral API assembly bootstrap for SDKWork Notary.

use axum::Router;
use sdkwork_notary_embedded_bootstrap::EmbeddedNotaryAssembly;
use sdkwork_web_bootstrap::{ApiAssemblyContribution, ReadinessCheck, ReadinessFuture};
use sdkwork_web_core::HttpRouteManifest;
use std::sync::Arc;

pub type ApiAssembly = ApiAssemblyContribution;

#[derive(Clone)]
struct EmbeddedNotaryReadiness {
    assembly: Arc<EmbeddedNotaryAssembly>,
}

impl ReadinessCheck for EmbeddedNotaryReadiness {
    fn check(&self) -> ReadinessFuture<'_> {
        let assembly = self.assembly.clone();
        Box::pin(async move {
            for pool in assembly.database_pools() {
                match pool.test_connection().await {
                    Ok(true) => {}
                    Ok(false) => {
                        return Err("notary database readiness query returned no row".to_owned());
                    }
                    Err(error) => {
                        return Err(format!("notary database readiness check failed: {error}"));
                    }
                }
            }
            Ok(())
        })
    }
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    let embedded = Arc::new(
        sdkwork_notary_embedded_bootstrap::assemble_embedded_notary_application_router_from_env()
            .await?,
    );
    let mut routes = Vec::new();
    routes.extend_from_slice(sdkwork_routes_notary_app_api::gateway_route_manifest().routes());
    routes.extend_from_slice(sdkwork_routes_notary_backend_api::gateway_route_manifest().routes());
    build_contribution(
        "SDKWork Notary API",
        embedded.router.clone(),
        HttpRouteManifest::from_owned_routes(routes),
        embedded,
    )
}

pub async fn assemble_app_api_contribution() -> Result<ApiAssembly, String> {
    let embedded = Arc::new(
        sdkwork_notary_embedded_bootstrap::assemble_embedded_notary_application_router_from_env()
            .await?,
    );
    build_contribution(
        "SDKWork Notary App API",
        embedded.app_router.clone(),
        sdkwork_routes_notary_app_api::gateway_route_manifest(),
        embedded,
    )
}

fn build_contribution(
    title: &str,
    router: Router,
    route_manifest: HttpRouteManifest,
    embedded: Arc<EmbeddedNotaryAssembly>,
) -> Result<ApiAssembly, String> {
    ApiAssemblyContribution::from_manifest(
        "sdkwork-notary",
        title,
        router,
        route_manifest,
        Vec::new(),
        Arc::new(EmbeddedNotaryReadiness { assembly: embedded }),
    )
}
