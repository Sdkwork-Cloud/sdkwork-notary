//! Embedded notary application bootstrap for platform runtime consumers.

mod adapters;
mod bootstrap;

pub use bootstrap::{
    assemble_embedded_notary_application_router,
    assemble_embedded_notary_application_router_from_env,
    assemble_embedded_notary_application_router_with_pool, EmbeddedNotaryAssembly,
    EmbeddedNotaryRuntimeConfig,
};
