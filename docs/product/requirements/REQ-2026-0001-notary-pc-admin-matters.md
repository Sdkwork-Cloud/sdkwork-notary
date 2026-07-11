# REQ-2026-0001 Notary PC Admin Matter Management

Status: implemented
Owner: SDKWork maintainers
Source: operator
Updated: 2026-07-11
Specs: REQUIREMENTS_SPEC.md, BACKEND_UI_SPEC.md, APP_PC_ARCHITECTURE_SPEC.md, APP_SDK_INTEGRATION_SPEC.md, API_SPEC.md, PAGINATION_SPEC.md, SECURITY_SPEC.md

## Problem

Internal notary operators require a dedicated PC backend-admin workflow to manage sellable notary matters. The implemented surface uses the existing backend contract and persists matter writes through the Merchandise owner boundary.

## Goals

- Provide an independent `backend-admin` package family for matter management.
- Model each matter as one Merchandise SPU and one SKU with `fulfillment_type=notary`.
- Support server-paginated search and status filtering, creation, editing, activation, and deactivation.
- Reuse the existing Notary backend API and composed backend SDK with the shared TokenManager.
- Reuse Order for case checkout/order snapshots and Payment only through Order-owned payment orchestration.
- Preserve Notary database ownership without adding matter, product, order, or payment tables.
- Remove embedded direct SQL access to Merchandise tables and provider-unavailable matter writes.

## Non-Goals

- A general Merchandise, Order, Payment, settlement, or refund operations console.
- Hard deletion of matter SKUs.
- A local Notary product, order, payment, or payment-status persistence model.
- Raw HTTP clients, manual authentication headers, generated transport edits, or frontend DTO forks.
- Arbitrary JSON editing for matter metadata.

## Users

- Internal notary administrators with `notary.matters.management.read`.
- Authorized operators with `notary.matters.create` or `notary.matters.update`.

## Acceptance Criteria

- `/admin/notary/matters` is owned by `sdkwork-notary-pc-admin-shell` and renders the independent `sdkwork-notary-pc-admin-merchandise` package.
- Backend SDK construction exists only in `sdkwork-notary-pc-admin-core`; app/user packages remain app-SDK-only.
- Matter list requests use generated SDK pagination and server-side `q` and `status` filters.
- Create requests use generated SDK idempotency support and create one SPU/SKU pair through the Merchandise owner service.
- Updates preserve SKU identity and update the owning SPU/SKU atomically through the Merchandise owner service.
- Runtime SPU/SKU ids are positive Snowflake values from `sdkwork-database-id`; deterministic hashes are limited to idempotent SPU/SKU business numbers.
- UI explicitly covers loading, empty, error, permission-denied, validation, and success states.
- Notary runtime contains no SQL that reads or writes `commerce_product_spu` or `commerce_product_sku`.
- No Notary DDL, migration, seed, schema contract, or table ownership is added for matters, orders, or payments. The shared database framework owns the reusable `sdkwork_node_registry` used for collision-free Snowflake node leases.
- Case creation continues to create Order-owned order and order-item records; payment intent, attempt, provider, and refund state remain Payment-owned behind Order orchestration.
- Component, layering, permission, API envelope, pagination, SDK import, TypeScript, Rust, database, contract, and full repository verification pass.

## Non-Functional Requirements

- Security: dual-token backend SDK, server-side permission and organization-scope enforcement, no browser credential duplication, idempotent create, safe operator-facing error mapping.
- Privacy: matter administration stores no new personal data; case PII ownership remains unchanged.
- Performance: default page size 20, maximum 200, bounded SQL pagination at the Merchandise owner repository, no client-side slicing of downloaded collections.
- Reliability: transactional SPU/SKU writes, deterministic idempotency conflict handling, collision-free Snowflake node leasing, fail-closed missing runtime configuration, no synthetic success responses.

## Affected Surfaces

- backend-api
- backend SDK
- Rust embedded runtime composition
- PC backend-admin
- documentation and component composition

## Trace

- Architecture: `docs/architecture/decisions/ADR-20260711-notary-admin-merchandise.md`
- API authority: `apis/backend-api/notary/notary-backend-api.openapi.json`
- SDK family: `sdks/sdkwork-notary-backend-sdk`
- PC packages: `apps/sdkwork-notary-pc/packages/sdkwork-notary-pc-admin-*`
- Runtime adapter: `crates/sdkwork-notary-embedded-bootstrap`
- Merchandise owner: `../sdkwork-merchandise/crates/sdkwork-merchandise-service` and `../sdkwork-merchandise/crates/sdkwork-merchandise-repository-sqlx`

## Verification

```text
node ../sdkwork-specs/tools/check-component-port-bindings.mjs --root . --strict
node ../sdkwork-specs/tools/check-application-layering.mjs --root .
node ../sdkwork-specs/tools/check-permission-composition.mjs --root .
node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .
node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
pnpm --dir apps/sdkwork-notary-pc typecheck
pnpm test:desktop
pnpm db:validate
pnpm sdk:check
pnpm verify
```
