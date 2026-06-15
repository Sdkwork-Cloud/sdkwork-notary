# SDKWork Routes Notary App API

Rust route crate for the SDKWork Notary App API surface.

It owns route path constants, Axum router composition, thin handlers, and a deterministic route manifest for `/app/v3/api/notary/*`. Business behavior is delegated to an injected Notary App API service port.
