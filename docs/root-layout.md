# SDKWork Notary Root Layout

This repository is a **contract-first domain library** governed by `sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`. It owns notary OpenAPI authorities, TypeScript SDK families, Rust route crates, and SQLx persistence. Host applications wire route layers; this repository does not ship a standalone deployable server.

## Active standard directories

| Directory | Status | Purpose |
| --- | --- | --- |
| `apis/` | Active | Author-owned OpenAPI authorities |
| `apps/` | Active | H5 mobile client at `apps/sdkwork-notary-h5/`; host integration pointers in `apps/README.md` |
| `crates/` | Active | Rust contract, service, repository, and route crates |
| `configs/` | Active | Topology profiles for host application wiring |
| `database/` | Active | Canonical database lifecycle assets (`database.manifest.json`, contract, baseline DDL, seeds) |
| `deployments/` | Reserved | Deployment manifests for host-packaged releases |
| `.sdkwork/` | Active | Repository-local skills, plugins, and ignored local state |
| `docs/` | Active | Topology summary, schema notes, root layout |
| `examples/` | Reserved | Runnable host wiring examples |
| `generated/` | Active | Materialized OpenAPI authorities (`generated/openapi/`) |
| `jobs/` | Reserved | Background workers |
| `plugins/` | Reserved | Repository-local plugins |
| `scripts/` | Active | Dev, manifest sync, contract verification |
| `sdks/` | Active | SDK families, route manifests, contract tests |
| `specs/` | Active | Root topology and component contracts |
| `tests/` | Pointer | Cross-cutting verification index |
| `tools/` | Reserved | Maintainer tooling |

## Intentionally absent or deferred

| Standard path | Notary decision |
| --- | --- |
| `apps/sdkwork-notary-h5/` | H5 mobile client root (`APP_H5_ARCHITECTURE_SPEC.md`) |
| `apps/sdkwork-notary/` (legacy) | Domain library catalog identity remains at root `sdkwork.app.config.json` |
| `services/*-standalone-gateway` | Route crates are consumed by host gateways, not a local API server |
| RPC / `sdkwork-discovery` | No gRPC services yet; discovery deferred until RPC is introduced |

## Framework integration summary

| Framework | Integration |
| --- | --- |
| `sdkwork-web-framework` | Route crates use `WebRequestContext`; `sdkwork-routes-notary-http-auth` mounts dual-token layers via `sdkwork-web-axum` and `sdkwork-iam-web-adapter`. |
| `sdkwork-database` | `sdkwork-notary-database-host` bootstraps lifecycle via `sdkwork-database-lifecycle` SPI; repository crate resolves `SDKWORK_NOTARY_*` config through `sdkwork-database-config`. |
| `sdkwork-utils` | `sdkwork-notary-case-contract` and `sdkwork-notary-case-repository-sqlx` use `sdkwork-utils-rust` for datetime formatting, base64 encoding, and SHA-256 hashing. |
| `sdkwork-discovery` | Not required (HTTP-only, no RPC). |

Verification: `pnpm verify` and `node --test scripts/verify-notary-standard-architecture.test.mjs`.
