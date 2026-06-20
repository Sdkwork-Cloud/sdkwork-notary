# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v1 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Project Purpose

`sdkwork-notary` is the contract-first notary domain library for SDKWork. It owns notary-specific persistence facts, OpenAPI authorities, TypeScript SDK families, and Rust runtime orchestration crates. Host applications wire route layers and dependency ports; this repository does not ship a standalone deployable server.

## Local Dictionary Structure

- `AGENTS.md`: local agent entrypoint and relative SDKWORK spec index.
- `.sdkwork/`: local skills, plugins, manifests, and workspace metadata (see `.sdkwork/README.md` and `.sdkwork/.gitignore`).
- `apis/`: API authority index and authored OpenAPI contracts for notary app/backend surfaces.
- `apps/`: pointer dictionary for host application roots (domain library has no local app).
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

## Spec Resolution Order

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `.sdkwork/README.md` when present.
3. Read `../sdkwork-specs/README.md` and the task-specific root specs.
4. Inspect implementation files only after the relevant dictionary entries are clear.

## Required Specs By Task Type

- Agent/workflow changes: `../sdkwork-specs/SOUL.md`, `../sdkwork-specs/AGENTS_SPEC.md`, `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`.
- Any code change: `../sdkwork-specs/CODE_STYLE_SPEC.md`, `../sdkwork-specs/NAMING_SPEC.md`, plus only the touched language/framework spec.
- Rust code: `../sdkwork-specs/RUST_CODE_SPEC.md`.
- HTTP route crates and web integration: `../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`, `../sdkwork-specs/WEB_BACKEND_SPEC.md`.
- TypeScript SDK and contract tests: `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`.
- API, SDK, database, runtime, security, and deployment changes must follow the task matrix in `../sdkwork-specs/README.md`.

## Code Style Rules

Read `../sdkwork-specs/CODE_STYLE_SPEC.md` and `../sdkwork-specs/NAMING_SPEC.md` before code changes.

For Rust, keep `src/lib.rs` limited to module declarations, re-exports, light docs, and wiring; move handlers, services, repositories, DTOs, SQL, provider clients, and tests into focused modules.

## Build, Test, and Verification

Run commands from this directory unless a command explicitly targets another path.

- `pnpm dev`: load the default self-hosted split-services development topology profile.
- `pnpm test:topology-validate`: validate `specs/topology.spec.json` against `@sdkwork/app-topology`.
- `pnpm test:contracts`: contract tests for OpenAPI, SDK packages, runtime standards, standard architecture, topology baggage, and optional Chat PC integration.
- `pnpm test:rust` or `cargo test --workspace --target-dir target-codex-test`: Rust workspace tests.
- `pnpm fmt:check` or `pnpm format:check`: verify Rust formatting for workspace crates.
- `pnpm db:validate`: validate `database/` assets against `DATABASE_FRAMEWORK_SPEC.md`.
- `pnpm test:topology-baggage`: scan active paths for retired topology vocabulary.
- `pnpm openapi:materialize` or `pnpm api:materialize`: copy authored `apis/` OpenAPI into `generated/openapi/` and sync framework metadata.
- `pnpm manifest:sync`: regenerate `sdks/_route-manifests` and sync framework metadata after route/OpenAPI changes.
- `pnpm verify`: run topology validation, topology baggage scan, database validation, contract tests, Rust formatting, and Rust tests.

Chat PC integration contract tests skip automatically when the real app root is absent. Set `SDKWORK_IM_PC_ROOT` (or legacy `SDKWORK_CHAT_PC_ROOT`) to point at `sdkwork-im/apps/sdkwork-im-pc` when validating cross-repo wiring.

Run the narrowest relevant check first, then broader verification when API contracts, SDK generation, persistence, security, or cross-package boundaries change.

## Agent Execution Rules

Use the convention dictionary instead of broad context loading. Keep changes scoped to the owning module, package, crate, or app root. Do not hand-edit generated SDK transport output. Do not replace generated SDK calls with raw HTTP. Record exact verification commands and important outputs before reporting completion.

## Human Review Rules

Request human review before breaking SDKWORK standards, changing public naming, altering security/auth behavior, changing database migrations or production deployment config, deleting data/files, or changing generated SDK ownership. Surface unresolved spec paths, app identity conflicts, component ownership conflicts, and API authority ambiguity instead of guessing.
