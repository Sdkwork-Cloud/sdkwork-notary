# SDKWork Notary PRD

Status: active
Owner: SDKWork maintainers
Application: sdkwork-notary
Updated: 2026-07-11
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## 1. Background And Problem

SDKWork Notary provides reusable contracts and runtime orchestration for enterprise notary services. It must let users submit and process cases while reusing platform-owned identity, merchandise, order, payment, and file capabilities. The product must not become a second system of record for those capabilities.

Internal operators also need a dedicated backend-admin experience for configuring sellable notary matters. Matters are commercial service definitions, so they use the Merchandise one-SPU/one-SKU model instead of a Notary-specific catalog table.

## 2. Target Users

- Applicants and organization users who submit notary cases.
- Notary staff who review, process, complete, or reject assigned cases.
- Notary administrators who configure organization access, staff assignments, and sellable matters.
- Platform operators and support staff who need auditable backend-admin workflows.

## 3. Goals And Non-Goals

Goals:

- Provide consistent app and backend contracts for notary access, matters, cases, parties, files, assignments, events, and reports.
- Reuse IAM organization membership and authorization, Merchandise SPU/SKU, Order checkout and order items, Payment execution, and Drive storage.
- Keep Notary persistence limited to organization profiles, cases, parties, assignments, and case events.
- Deliver a production-capable PC backend-admin matter management workflow with explicit package and SDK boundaries.
- Preserve tenant isolation, organization scope, auditability, idempotency, bounded pagination, and encrypted case PII.

Non-goals:

- Owning a duplicate staff directory, merchandise catalog, order ledger, payment ledger, or file store.
- Adding `notary_matter`, `notary_order`, `notary_payment`, or equivalent database tables.
- Allowing app/user packages to call backend APIs or allowing UI packages to bypass generated SDKs.
- Providing a general commerce operations console inside Notary.

## 4. Scope

In scope:

- Organization-level notary enablement and access checks.
- SKU-backed matter discovery and backend administration.
- Case submission, order linkage, parties, Drive folder/file linkage, assignment, workflow transitions, and timeline events.
- Backend case management, staff discovery, and reports.
- PC app/user workspace and PC internal backend-admin matter management.

Data ownership stays with the canonical owner. Notary stores stable ids and immutable business snapshots only where needed for case performance and auditability.

## 5. User Scenarios

1. A verified organization enables notary service and receives a Drive notary space.
2. An authorized operator creates or updates a matter; Merchandise transactionally persists its SPU/SKU representation.
3. An applicant selects an active matter and submits a case; Order creates the order and order item, while Notary stores the case and price snapshot.
4. Order invokes Payment for payment execution when payment is required; Notary never creates a parallel payment record.
5. Notary staff process the case using IAM assignments and Drive-backed materials with auditable case events.
6. An operator filters, searches, activates, or deactivates matters in the PC backend-admin without downloading the full catalog.

## 6. Success Metrics

- 100% of matter writes pass through the Merchandise owner service; zero Notary SQL references to Merchandise tables.
- 100% of case-created commercial records use Order-owned order and order-item persistence.
- Zero Notary-owned product, order, payment, or file tables.
- All protected APIs use dual-token authorization, declared permissions, organization data scope, and audit metadata.
- All interactive lists use server pagination with bounded page sizes and no client-side slice pagination.
- Repository verification, SDK generation, Rust tests, PC typecheck/build, and database ownership checks pass before release.

## 7. Phases

- Phase 1 (completed): contract-first Notary domain, app/backend APIs, generated SDKs, persistence, and app PC/H5 clients.
- Phase 2 (completed): PC backend-admin package family and Merchandise-backed matter management.
- Phase 3 (remaining): new operational capabilities that require separately reviewed owner-domain contracts. The current case-acceptance payment gate is implemented through the Order owner state.

## 8. Linked Requirements

- [REQ-2026-0001 Notary PC Admin Matter Management](../../requirements/REQ-2026-0001-notary-pc-admin-matters.md)

## 9. Open Questions

- No open architecture questions block the current matter-management scope. The Order-owned paid-state contract is available, including `paid`, `payment_status`, `paid_at`, and `paid_amount`; case acceptance now queries that owner state and rejects unpaid or terminal orders. Additional payment-dependent transitions or refund automation require separate reviewed requirements.
