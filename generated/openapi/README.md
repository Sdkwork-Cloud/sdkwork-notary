# Generated OpenAPI Authorities

Owner OpenAPI authority documents materialized from authored contracts in `apis/`.

| Authority | Source | Consumers |
| --- | --- | --- |
| `notary-app-api.openapi.json` | `apis/app-api/notary/notary-app-api.openapi.json` | `sdkwork-notary-app-sdk`, route crate manifests |
| `notary-backend-api.openapi.json` | `apis/backend-api/notary/notary-backend-api.openapi.json` | `sdkwork-notary-backend-sdk`, route crate manifests |

Refresh after contract edits:

```powershell
pnpm api:materialize
```
