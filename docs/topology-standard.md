# SDKWork Notary Topology

Archetype: `application-http-gateway` (`specs/topology.spec.json`, `schemaVersion: 4`).

Platform standard: `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md`

## Default dev profile

`standalone.development`: load the profile with:

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

PC browser client:

```bash
pnpm dev:desktop
```

## Surfaces

| Surface id | Plane | Consumer |
| --- | --- | --- |
| `application.public-ingress` | application | Notary App SDK (`/app/v3/api/notary`) |
| `application.backend-http` | application | Notary Backend SDK (`/backend/v3/api/notary`) |
| `platform.api-gateway` | platform | IAM, Merchandise, Order, Payment, and Drive owner surfaces via `sdkwork-api-cloud-gateway` |

`sdkwork-notary` is a domain library. It declares topology authority and profile env, but host applications wire `sdkwork-routes-notary-app-api` and `sdkwork-routes-notary-backend-api` into their own HTTP servers.

Loader: `scripts/lib/notary-topology.mjs` → `@sdkwork/app-topology`.

Validate:

```bash
pnpm test:topology-validate
```
