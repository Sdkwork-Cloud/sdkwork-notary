//! In-process notary application bootstrap for unified-process platform consumers.

mod adapters;
mod bootstrap;

pub use bootstrap::{
    assemble_embedded_notary_application_router,
    assemble_embedded_notary_application_router_from_env, EmbeddedNotaryAssembly,
    EmbeddedNotaryRuntimeConfig,
};
