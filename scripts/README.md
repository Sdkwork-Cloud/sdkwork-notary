# Scripts

Development, manifest sync, and verification scripts for the notary domain library.

| Script | Purpose |
| --- | --- |
| `notary-dev.mjs` | Load topology profiles for local host wiring |
| `sync-notary-openapi-authorities.mjs` | Materialize `apis/` contracts into `generated/openapi/` |
| `generate-notary-route-manifests.mjs` | Generate `sdks/_route-manifests` from OpenAPI and `routes.rs` |
| `sync-notary-api-framework-metadata.mjs` | Sync `x-sdkwork-request-context`, `x-sdkwork-api-surface`, `x-sdkwork-auth-mode`, and route source metadata |
| `verify-notary-standard-architecture.test.mjs` | SDKWork workspace and framework alignment gate |

See `scripts/dev/` for topology baggage checks.
