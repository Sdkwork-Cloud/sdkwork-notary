# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v1 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

- App key: `sdkwork-notary`
- Manifest: `sdkwork.app.config.json`
- Runtime family: `library` (contract-first domain library; host apps wire route layers)
- Dev client surface: `apps/sdkwork-notary-h5`

Read `sdkwork.app.config.json` when changing application identity, release metadata, runtime config, SDK wiring, or packaging behavior.

## Local Dictionary Structure

- `AGENTS.md`: local agent entrypoint and relative SDKWORK spec index.
- `.sdkwork/`: local skills, plugins, manifests, and workspace metadata (see `.sdkwork/README.md` and `.sdkwork/.gitignore`).
- `apis/`: API authority index and authored OpenAPI contracts for notary app/backend surfaces.
- `apps/`: H5 mobile client at `apps/sdkwork-notary-h5/` plus host integration pointers
- `configs/topology/`: v2 runtime topology profile env files.
- `deployments/`, `jobs/`, `tools/`, `plugins/`, `examples/`, `tests/`: standard workspace directories (see `docs/root-layout.md`).
- `docs/topology-standard.md`: human topology summary and dev commands.
- `docs/root-layout.md`: workspace directory dictionary and framework integration summary.
- `generated/openapi/`: owner OpenAPI authority documents.
- `sdks/`: SDK families, route manifests, composed facades, and contract tests.
- `crates/`: Rust contract, service, repository, and route crates.
- `sdkwork.app.config.json`: SDKWork application identity manifest.
- `sdkwork.workflow.json`: release and packaging workflow for domain library verification.
- `package.json` / `Cargo.toml`: workspace manifests.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Spec Resolution Order

Use dynamic progressive loading:

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json` only when app behavior, runtime config, SDK wiring, release, or packaging is touched.
3. Read local `specs/README.md` and `specs/component.spec.json` only when local contracts are relevant.
4. Read `.sdkwork/README.md`, `.sdkwork/skills/`, and `.sdkwork/plugins/` only when local agent extensions are relevant.
5. Read `../sdkwork-specs/README.md`, then only the task-specific root specs required by the current task.
6. Inspect implementation files only after the relevant dictionary entries are clear.

Language-specific specs are on-demand; do not eagerly load Rust, TypeScript, and frontend specs for unrelated work.

## Required Specs By Task Type

- Agent/workflow changes: `../sdkwork-specs/SOUL.md`, `../sdkwork-specs/AGENTS_SPEC.md`, `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`.
- Any code change: `../sdkwork-specs/CODE_STYLE_SPEC.md`, `../sdkwork-specs/NAMING_SPEC.md`, plus only the touched language/framework spec.
- Rust code: `../sdkwork-specs/RUST_CODE_SPEC.md`.
- HTTP route crates and web integration: `../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`, `../sdkwork-specs/WEB_BACKEND_SPEC.md`.
- TypeScript SDK and contract tests: `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`.
- pnpm script surfaces: `../sdkwork-specs/PNPM_SCRIPT_SPEC.md`.
- API, SDK, database, runtime, security, and deployment changes must follow the task matrix in `../sdkwork-specs/README.md`.
- GitHub packaging workflows: `../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`.
- Security/auth changes: `../sdkwork-specs/IAM_SPEC.md`, `../sdkwork-specs/SECURITY_SPEC.md`, `../sdkwork-specs/IAM_MODULE_MANIFEST_SPEC.md`.

## Code Style Rules

Read `../sdkwork-specs/CODE_STYLE_SPEC.md` and `../sdkwork-specs/NAMING_SPEC.md` before code changes.

For Rust, keep `src/lib.rs` limited to module declarations, re-exports, light docs, and wiring; move handlers, services, repositories, DTOs, SQL, provider clients, and tests into focused modules.

Build scripts, dev runners, and `pnpm clean` must follow `CODE_STYLE_SPEC.md` §7 (Build Source Integrity And Self-Healing). Git-tracked build-critical source files must be verified before builds and self-healed from git when missing; `clean` must not delete them.

## Build, Test, and Verification

Run commands from this directory unless a command explicitly targets another path.

- `pnpm dev`: load the default standalone split-services development topology profile.
- `pnpm test:topology-validate`: validate `specs/topology.spec.json` against `@sdkwork/app-topology`.
- `pnpm test:contracts`: contract tests for OpenAPI, SDK packages, runtime standards, standard architecture, topology baggage, and optional Chat PC integration.
- `pnpm test:rust` or `cargo test --workspace --target-dir target-codex-test`: Rust workspace tests.
- `pnpm format:check`: verify Rust formatting for workspace crates.
- `pnpm db:validate`: validate `database/` assets against `DATABASE_FRAMEWORK_SPEC.md`.
- `pnpm test:topology-baggage`: scan active paths for retired topology vocabulary.
- `pnpm api:materialize`: copy authored `apis/` OpenAPI into `generated/openapi/`, regenerate route manifests, and sync framework metadata.
- `pnpm verify`: run topology validation, topology baggage scan, database validation, contract tests, Rust formatting, and Rust tests.

Chat PC integration contract tests skip automatically when the real app root is absent. Set `SDKWORK_IM_PC_ROOT` to point at `sdkwork-im/apps/sdkwork-im-pc` when validating cross-repo wiring.

Run the narrowest relevant check first, then broader verification when API contracts, SDK generation, persistence, security, or cross-package boundaries change.

## Agent Execution Rules

Use the convention dictionary instead of broad context loading. Keep changes scoped to the owning module, package, crate, or app root. Do not hand-edit generated SDK transport output. Do not replace generated SDK calls with raw HTTP. Record exact verification commands and important outputs before reporting completion.

## HTTP API Response Envelope

All L2+ `app-api`, `backend-api`, and SDKWork-owned business `open-api` HTTP contracts `MUST` follow `API_SPEC.md` section 4.5, section 14, and section 15:

- **Input:** typed request bodies, section 14.1 list/search/command input, `SdkWorkListQuery`, and `q` for free-text search.
- **Success output:** `SdkWorkApiResponse` with `{ "code": 0, "data": <payload>, "traceId": "<server-uuid>" }`.
- **Error output:** HTTP 4xx/5xx `application/problem+json` (`ProblemDetail`) with numeric `code` and `traceId`.
- Success `code` is numeric `int32`; HTTP 2xx JSON bodies `MUST` use `0` only. REST semantics remain on HTTP status (`201`, `202`, etc.).
- Platform error codes are numeric non-zero values per section 15.3 (`40001`, `40101`, `40401`, …).
- Single resource: `data.item`
- Lists: `data.items` + `data.pageInfo` (`PageInfo.mode` is `offset` or `cursor`)
- Commands: `data.accepted` plus optional `resourceId` / `status`
- Async accept (`202`): `data.operationId`, `data.status`, optional `pollUrl`

Vendor compatibility `open-api` routes that mirror upstream tool or provider wire (for example OpenAI `/v1/*`, Claude Code, Codex) `MAY` opt out only when every exempt operation declares `x-sdkwork-wire-protocol: external` and `x-sdkwork-external-protocol-id` per `API_SPEC.md` section 4.5.2. SDKWork-owned business `open-api` operations `MUST NOT` opt out.

Errors `MUST` use HTTP 4xx/5xx with `application/problem+json` (`ProblemDetail`) including required numeric `code` and `traceId`. Business failures `MUST NOT` use HTTP 2xx with non-zero `code`, string wire codes, `success`, or human `message`.

Forbidden legacy envelopes and fields: `PlusApiResult`, `AppbaseApiResult`, `StoreApiResult`, `SdkWorkResponse`, per-domain `*ApiResult`, wire field `requestId`, bare domain DTOs at the HTTP root, and top-level `{ items, pageInfo, traceId }` without `data`.

Handlers `MUST` serialize success and map errors through `sdkwork-web-framework` response mapping. Generated HTTP SDKs (`--standard-profile sdkwork-v3`) unwrap `data` by default and expose typed numeric `ProblemDetail.code` / `traceId` on errors; use `.raw` when the full envelope is required.

Before completing API contract, SDK generation, or frontend service work, run:

```bash
node <sdkwork-specs>/tools/check-api-response-envelope.mjs --workspace <workspace-root>
```

Authority: `sdkwork-specs/API_SPEC.md` section 4.5 and sections 14–16, `SDK_SPEC.md` section 4.2, `FRONTEND_SPEC.md`, `MIGRATION_SPEC.md` section 4.2.

## Human Review Rules

Request human review before breaking SDKWORK standards, changing public naming, altering security/auth behavior, changing database migrations or production deployment config, deleting data/files, or changing generated SDK ownership. Surface unresolved spec paths, app identity conflicts, component ownership conflicts, and API authority ambiguity instead of guessing.
