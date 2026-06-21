# Route Manifests

Canonical `sdkwork.route.manifest` documents for Notary HTTP route crates.

| File | Route crate | API surface |
| --- | --- | --- |
| `app-api/sdkwork-router-notary-app-api.route-manifest.json` | `sdkwork-router-notary-app-api` | `app-api` |
| `backend-api/sdkwork-router-notary-backend-api.route-manifest.json` | `sdkwork-router-notary-backend-api` | `backend-api` |

Regenerate after OpenAPI or `routes.rs` changes:

```powershell
node scripts/generate-notary-route-manifests.mjs
node scripts/sync-notary-api-framework-metadata.mjs
```

Or run `pnpm api:materialize`.
