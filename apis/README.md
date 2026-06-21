# Notary API Authorities

Authoritative HTTP contracts for this repository:

| Surface | Authored contract | Generated authority | Route prefix |
| --- | --- | --- | --- |
| App | `apis/app-api/notary/notary-app-api.openapi.json` | `generated/openapi/notary-app-api.openapi.json` | `/app/v3/api/notary` |
| Backend | `apis/backend-api/notary/notary-backend-api.openapi.json` | `generated/openapi/notary-backend-api.openapi.json` | `/backend/v3/api/notary` |

Keep authored contracts in `apis/` aligned with `generated/openapi/` when contracts change:

```powershell
pnpm api:materialize
pnpm sdk:generate
```

Generated TypeScript transport SDKs live under `sdks/sdkwork-notary-app-sdk` and `sdks/sdkwork-notary-backend-sdk`.

Rust route dispatchers:

- `crates/sdkwork-router-notary-app-api`
- `crates/sdkwork-router-notary-backend-api`

Both dispatch to `sdkwork-notary-case-service` over explicit Appbase, Commerce, Drive, and Notary repository ports.
