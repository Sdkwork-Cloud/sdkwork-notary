# SDKWork Routes Notary Backend API

Rust route crate for the SDKWork Notary Backend API surface.

It owns operator route path constants, Axum router composition, thin handlers, and a deterministic route manifest for `/backend/v3/api/notary/*`. Business behavior is delegated to an injected Notary Backend API service port.
