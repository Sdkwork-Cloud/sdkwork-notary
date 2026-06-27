//! Gateway assembly for sdkwork-notary.
//! Business routes are composed through embedded bootstrap for platform consumers.

mod generated;

pub struct ApplicationAssembly {
    pub router: axum::Router,
}

pub async fn assemble_application_business_router() -> Result<ApplicationAssembly, String> {
    let assembly =
        sdkwork_notary_embedded_bootstrap::assemble_embedded_notary_application_router_from_env()
            .await?;
    Ok(ApplicationAssembly {
        router: assembly.router,
    })
}

pub async fn assemble_application_router() -> Result<ApplicationAssembly, String> {
    assemble_application_business_router().await
}

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
