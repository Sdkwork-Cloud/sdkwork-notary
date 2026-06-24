> Migrated from `docs/topology-standard.md` on 2026-06-24.
> Owner: SDKWork maintainers

Archetype: `application-http-gateway` (`specs/topology.spec.json`, `schemaVersion: 2`).

Platform standard: `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md`

## Default dev profile

`standalone.split-services.development` — load the profile with:

```bash
pnpm dev
```

Cloud development profile:

```bash
pnpm dev:cloud
```

H5 browser client:

```bash
pnpm dev:browser
```

## Surfaces

| Surface id | Plane | Consumer |
| --- | --- | --- |
| `application.public-ingress` | application | Notary App SDK (`/app/v3/api/notary`) |
| `application.backend-http` | application | Notary Backend SDK (`/backend/v3/api/notary`) |
| `platform.api-gateway` | platform | Appbase, Commerce, Drive SDKs via `sdkwork-api-cloud-gateway` |

`sdkwork-notary` is a domain library. It declares topology authority and profile env, but host applications wire `sdkwork-router-notary-app-api` and `sdkwork-router-notary-backend-api` into their own HTTP servers.

Loader: `scripts/lib/notary-topology.mjs` → `@sdkwork/app-topology`.

Validate:

```bash
pnpm test:topology-validate
```

