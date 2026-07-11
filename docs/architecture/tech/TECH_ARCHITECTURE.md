# SDKWork Notary Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-11
Specs: ARCHITECTURE_DECISION_SPEC.md, DOCUMENTATION_SPEC.md, APPLICATION_LAYERED_ARCHITECTURE_SPEC.md, APP_PC_ARCHITECTURE_SPEC.md, APP_SDK_INTEGRATION_SPEC.md, API_SPEC.md, PAGINATION_SPEC.md, DATABASE_SPEC.md, SECURITY_SPEC.md

## Document Map

- [TECH-2026-06-10-notary-contract-implementation.md](TECH-2026-06-10-notary-contract-implementation.md)
- [TECH-root-layout.md](TECH-root-layout.md)
- [TECH-topology-standard.md](TECH-topology-standard.md)
- [ADR-20260711 Notary Admin Uses Merchandise, Order, And Payment Owners](../decisions/ADR-20260711-notary-admin-merchandise.md)

## 1. Architecture Overview

Architecture detail lives in the linked TECH shards below.

`sdkwork-notary` is a contract-first Rust domain library with authored OpenAPI authorities, generated SDK families, SQLx Notary repositories, embedded runtime adapters, and independently runnable H5 and PC application roots. It composes stable owner capabilities through ports:

```text
PC backend-admin
  -> @sdkwork/notary-backend-sdk
  -> Notary backend route/service
  -> Merchandise owner service (SPU/SKU)

Case submission
  -> Notary case service
  -> Order owner (checkout/order/order item)
  -> Payment owner through Order orchestration
  -> Drive owner (space/folder/files)
  -> IAM owner (member and permission context)
```


## 2. Technology Choices

- Rust 2021, async traits, Axum route adapters, and SQLx repositories.
- SDKWork web framework response/error mapping for `sdkwork-v3` envelopes and RFC 9457 problems.
- Authored OpenAPI under `apis/`, materialized authority under `generated/openapi/`, and generator-owned transport under `sdks/**/generated/server-openapi`.
- React 19 and React Router for PC surfaces; backend-admin packages use the generated Notary backend SDK.
- `sdkwork-utils-rust` and `@sdkwork/utils` for shared pagination, parsing, and common utilities where owner APIs exist.
- `sdkwork-database-id` for positive Snowflake runtime ids and collision-free node leases held by the assembly lifecycle.

## 3. System Boundaries And Modules

- `sdkwork-notary-case-contract`: Notary records, status values, typed errors, and runtime context.
- `sdkwork-notary-case-service`: use cases and ports; no SQL or HTTP transport ownership.
- `sdkwork-notary-case-repository-sqlx`: Notary-owned persistence only.
- `sdkwork-notary-embedded-bootstrap`: concrete IAM, Merchandise/Order, and Drive adapters for same-process deployment.
- `sdkwork-routes-notary-app-api` and `sdkwork-routes-notary-backend-api`: surface-specific HTTP adaptation.
- `sdkwork-notary-app-sdk` and `sdkwork-notary-backend-sdk`: owner-only generated client families plus authored composed facades.
- `apps/sdkwork-notary-pc`: shared renderer with separate app and backend-admin package families.

Merchandise owns SPU/SKU validation and SQL. Order owns checkout, order, order item, amount snapshot, cancellation, and payment orchestration. Payment owns payment intent, attempts, provider credentials, callbacks, and refunds. Notary owns the case workflow and stores only stable references and case snapshots. Before case acceptance, Notary queries the Order owner state and requires a non-terminal order with successful payment, except for zero-total orders; it never copies payment state into its database.

## 4. Directory And Package Layout

The PC application uses these package families:

- `sdkwork-notary-pc-core`, `sdkwork-notary-pc-shell`, and `sdkwork-notary-pc-notary` for app/user workflows.
- `sdkwork-notary-pc-admin-core` for backend SDK construction and operator permission context.
- `sdkwork-notary-pc-admin-shell` for `/admin` layout, navigation, and route guards.
- `sdkwork-notary-pc-admin-merchandise` for `/admin/notary/matters` pages, services, hooks, routes, i18n, and view state.

Every authored package owns `specs/component.spec.json`; package imports use public exports only.

## 5. API, SDK, And Data Ownership

- App API prefix: `/app/v3/api/notary`; app clients consume `@sdkwork/notary-app-sdk`.
- Backend API prefix: `/backend/v3/api/notary`; backend-admin clients consume `@sdkwork/notary-backend-sdk`.
- Matter list/create/update remain Notary orchestration operations with `notary.matters.*` permissions, but physical SPU/SKU persistence is delegated to the Merchandise owner service.
- List/search output is `{ items, pageInfo }`; matter filters include `q`, `status`, organization scope, and bounded cursor pagination.
- Notary database ownership is limited to `notary_organization_profile`, `notary_case`, `notary_party`, `notary_case_assignment`, and `notary_case_event`.
- There are no Notary matter, order, payment, staff-directory, or file tables and no copied dependency DDL.

## 6. Security, Privacy, And Observability

- App and backend routes use dual-token request context, tenant isolation, organization data scope, and server-side permission checks.
- The PC root creates one TokenManager; app SDKs bind through app core and backend SDKs bind only through admin core.
- UI permission hints control navigation and command affordances only; backend authorization remains authoritative.
- Matter creation uses generated SDK idempotency options. Owner services reject conflicting replay payloads.
- Merchandise SPU/SKU ids come from the injected SDKWork ID provider. SHA-256 material is restricted to stable idempotent SPU/SKU numbers, never primary keys.
- Case identity/contact fields remain encrypted or hashed in Notary persistence; matter administration adds no PII.
- Case acceptance fails closed for unpaid or terminal Order states; late payment on a terminal Order remains auditable through the Order-owned lifecycle event.
- Runtime operations log operation id and scoped identifiers without credentials or decrypted PII. Provider and storage errors are normalized at adapters.

## 7. Deployment And Runtime Topology

Standalone deployment mounts Notary route crates and concrete owner adapters in one process while preserving domain ports. Cloud/split deployment may replace those adapters with generated SDK or RPC clients. Both modes expose the same API authorities and keep per-owner database lifecycle modules independent.

Runtime config derives separate public and backend API roots from topology profiles and fails closed when required production endpoints are missing. A shared gateway profile may intentionally supply one root for both surfaces.

## 8. Architecture Decision Index

- [ADR-20260711-notary-admin-merchandise](../decisions/ADR-20260711-notary-admin-merchandise.md): backend-admin package split, Merchandise ownership, Order/Payment reuse, and no-new-database decision.

## 9. Verification

Verification includes component/port binding, application layering, frontend composition, permission composition, route ownership, API operation/envelope, pagination, SDK consumer imports, idempotent generation, Rust formatting/tests, PC typecheck/build, database ownership checks, and full `pnpm verify`.
