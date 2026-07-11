# ADR-20260711 Notary Admin Uses Merchandise, Order, And Payment Owners

Status: accepted
Requirement: REQ-2026-0001
Owner: SDKWork maintainers
Date: 2026-07-11
Specs: ARCHITECTURE_DECISION_SPEC.md, DOMAIN_SPEC.md, APP_PC_ARCHITECTURE_SPEC.md, BACKEND_UI_SPEC.md, APP_SDK_INTEGRATION_SPEC.md, DATABASE_SPEC.md, SECURITY_SPEC.md

## Context

Notary matters are sellable service definitions. SDKWork defines `commerce/merchandise` as the owner of SPU/SKU master data, Order as the owner of order and order-item state, and Payment as the owner of payment execution state. Notary stores case references and price snapshots while all SPU/SKU reads and writes remain behind the Merchandise owner boundary.

The PC root also needs to host app/user and internal operator workflows without allowing app packages to construct backend SDK clients.

## Decision

1. Add three independent PC backend-admin packages:
   - `@sdkwork/notary-pc-admin-core` for backend SDK construction, shared TokenManager binding, operator context, permission checks, and audit metadata.
   - `@sdkwork/notary-pc-admin-shell` for `/admin` layout, navigation, route guards, and route assembly.
   - `@sdkwork/notary-pc-admin-merchandise` for the operator-facing "Matter Management" workflow at `/admin/notary/matters`.
2. Keep the UI's only business transport dependency as `@sdkwork/notary-backend-sdk`. The feature package does not call Merchandise, Order, or Payment SDKs directly.
3. Keep the Notary backend API as the application orchestration authority for notary-specific permissions and organization scope.
4. Move physical SPU/SKU list and write behavior behind a reusable Merchandise owner service and SQLx repository. The owner repository performs bounded filtering and transactional one-SPU/one-SKU writes. Notary contains no Merchandise SQL.
5. Continue using Order for checkout, order, order-item, pricing snapshot, cancellation, and payment orchestration. Payment remains behind Order and owns payment intent, attempt, provider, webhook, and refund execution facts.
6. Do not store a parallel Notary payment status. Case acceptance queries the Order-owned payment and lifecycle state, requiring successful payment or a zero-total order while rejecting terminal orders.
7. Do not add Notary tables, migrations, copied schemas, or dependency DDL. Existing logical references remain unchanged.
8. Generate runtime SPU/SKU ids through `sdkwork-database-id`. Keep the framework-owned Snowflake node lease alive in the application assembly; deterministic hashes are used only for idempotent business numbers.

## Alternatives

- Add `notary_matter`, `notary_order`, or `notary_payment` tables: rejected because it duplicates owner domains and violates the explicit no-new-database constraint.
- Let the PC feature call Merchandise, Order, and Payment SDKs directly: rejected because it leaks cross-domain orchestration into UI and duplicates permission and token handling.
- Keep embedded direct SQL in Notary: rejected because it bypasses owner validation, idempotency, transaction, and schema evolution boundaries.
- Use the retired generic Commerce backend SDK: rejected because current SDKWork consumer imports require domain/composed SDK families and `merchandise` is the canonical capability.
- Put operator pages in the existing app shell/core packages: rejected because app/user packages may not import backend SDKs and `/admin` path alone does not classify a component as `backend-admin`.

## Consequences

- The PC root gains explicit app and admin surfaces with separate SDK and route ownership.
- Merchandise owns all SPU/SKU SQL and can evolve its persistence without Notary repository changes.
- Notary preserves its domain-specific API, permissions, audit events, and organization scope while remaining persistence-light.
- Order and Payment remain reusable across future notary channels and client applications.
- The embedded runtime gains sibling Merchandise crate dependencies, but no copied database lifecycle or schema definitions.
- The shared database framework may maintain its reusable `sdkwork_node_registry`; Notary does not define or own that infrastructure table.
- Split-service mode can replace the embedded Merchandise adapter with a generated owner SDK adapter without changing Notary service contracts or the PC feature.

## Verification

- Component and frontend layering validators prove package ownership and SDK boundaries.
- API and pagination validators prove canonical filters, envelopes, and server pagination.
- Rust tests prove owner repository transactions, idempotency, filtering, and Notary adapter mapping.
- Database validation proves that no new Notary-owned matter, order, or payment definitions exist.
- Static scans prove no raw HTTP, generated transport consumer import, manual auth header, or Notary Merchandise SQL remains.

## Supersedes / Superseded By

The embedded Notary adapter consumes the Merchandise owner service and contains no SPU/SKU SQL. Active requirements, architecture, SDK, and runtime documentation use this owner boundary consistently.
