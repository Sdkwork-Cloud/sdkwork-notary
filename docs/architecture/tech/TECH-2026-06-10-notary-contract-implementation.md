# Notary Contract And Owner Integration

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-11
Requirement: REQ-2026-0001
Decision: ADR-20260711-notary-admin-merchandise
Specs: ARCHITECTURE_DECISION_SPEC.md, DOMAIN_SPEC.md, API_SPEC.md, SDK_SPEC.md, DATABASE_SPEC.md, SECURITY_SPEC.md

## Purpose

This shard records the current Notary contract and owner integration. It replaces the initial contract-only implementation notes; historical task checklists remain under `docs/superpowers/plans/` and are not active architecture authority.

## Current Boundaries

Notary owns organization-level enablement, case dossiers, parties, assignments, and case events. It references IAM, Merchandise, Order, Payment, and Drive by stable ids and published ports.

Merchandise is the source of truth for sellable notary matters. A matter is represented by one Merchandise SPU and one SKU with `fulfillment_type=notary` and the standard `notary` product classification. The Merchandise owner service performs validation, bounded list/search, idempotency conflict checks, and transactional SPU/SKU writes, including `spec_json`.

Order is the source of truth for checkout sessions, quotes, orders, order items, pricing snapshots, cancellation, and payment orchestration. Payment is the source of truth for payment intents, attempts, providers, callbacks, and refunds. Notary does not create parallel order or payment records and does not persist a parallel payment status.

Drive owns notary spaces, case folders, file nodes, uploads, and download grants. IAM owns organization members, roles, positions, departments, sessions, and permissions.

## API And SDK

- App authority: `apis/app-api/notary/notary-app-api.openapi.json`, `/app/v3/api/notary`, `@sdkwork/notary-app-sdk`.
- Backend authority: `apis/backend-api/notary/notary-backend-api.openapi.json`, `/backend/v3/api/notary`, `@sdkwork/notary-backend-sdk`.
- Matter operations: `notary.matters.management.list`, `notary.matters.create`, and `notary.matters.update`.
- Matter list filters: `page_size`, opaque `cursor`, `q`, `organization_id`, and `status`; response data is `{ items, pageInfo }`.
- Backend consumers use the composed SDK facade. Generated transport remains generator-owned under `generated/server-openapi`.

## Runtime Composition

The embedded standalone bootstrap injects concrete adapters into the Notary case service:

```text
Notary routes -> Notary case service ports
  -> Merchandise single-SKU owner service for matter CRUD/list
  -> Order service/repository for case orders
  -> Drive service for spaces/folders/files
  -> IAM service/repository for organization context
```

The same ports can be replaced by split-service SDK/RPC adapters without changing route contracts. Notary runtime code must not contain SQL against owner tables or mount copied owner routes.

## Persistence

The Notary database contract contains only:

- `notary_organization_profile`
- `notary_case`
- `notary_party`
- `notary_case_assignment`
- `notary_case_event`

Merchandise, Order, Payment, Drive, and IAM tables are dependency-owned. Notary schema registry entries are read-only cross-domain references; no matter, product, order, or payment DDL is added here.

## PC Surfaces

The PC root keeps app/user packages separate from backend-admin packages:

- `/notary`: `pc-core`, `pc-shell`, and `pc-notary`, app SDK only.
- `/admin`: `pc-admin-core`, `pc-admin-shell`, and `pc-admin-merchandise`, backend SDK only.

`pc-admin-merchandise` exposes Matter Management at `/admin/notary/matters`, requests one server page at a time, and uses generated SDK methods for create/update/idempotency. It never constructs an SDK client or sends raw HTTP.

## Security And Verification

All protected operations use dual-token request context, tenant and organization scope, declared Notary permissions, audit metadata, and normalized `ProblemDetail` errors. The PC app binds one TokenManager; only `pc-admin-core` may construct the backend client.

The active verification set is documented in REQ-2026-0001 and includes component/port, layering, permission, SDK import, API envelope, pagination, Rust, database, PC, generation-idempotence, and full repository checks.
