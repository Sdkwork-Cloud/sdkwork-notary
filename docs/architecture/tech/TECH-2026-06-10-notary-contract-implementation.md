> Migrated from `docs/superpowers/plans/2026-06-10-notary-contract-implementation.md` on 2026-06-24.
> Owner: SDKWork maintainers

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the initial `sdkwork-notary` contract layer with reusable database tables, app/backend OpenAPI, SDK family metadata, and composed SDK facades that integrate Appbase, Commerce, and Drive without duplicating their owned APIs.

**Architecture:** Notary owns only its case dossier, party, assignment, event, and organization profile tables. Commerce remains the source of truth for SKU-backed notary matters and orders, Drive remains the source of truth for files/folders under `space_type='notary'`, and Appbase/IAM remains the source of truth for organization members, roles, positions, and departments.

**Tech Stack:** SQLite-compatible SQL migrations, schema registry YAML, OpenAPI 3.1.2 JSON, SDKWork SDK family manifests, Node contract tests, TypeScript authored composed facades.

---

### Task 1: Contract Tests

**Files:**
- Create: `package.json`
- Create: `sdks/test/notary-contracts.test.mjs`

- [x] **Step 1: Write failing tests**

The contract tests assert the database, OpenAPI, SDK family, and composed facade boundaries.

- [ ] **Step 2: Run test to verify it fails**

Run: `pnpm test`
Expected: FAIL because the SQL, OpenAPI, SDK manifests, and composed facade files are not implemented yet.

### Task 2: Database Contract

**Files:**
- Create: `crates/sdkwork-notary-case-repository-sqlx/migrations/0001_notary_foundation.sql`
- Create: `docs/schema-registry/tables/001-notary-core.yaml`

- [ ] **Step 1: Implement minimal SQL**

Create `notary_organization_profile`, `notary_case`, `notary_party`, `notary_case_assignment`, and `notary_case_event`.

- [ ] **Step 2: Verify tests**

Run: `pnpm test`
Expected: database assertions pass, OpenAPI/SDK assertions still fail until later tasks are complete.

### Task 3: OpenAPI Contracts

**Files:**
- Create: `generated/openapi/notary-app-api.openapi.json`
- Create: `generated/openapi/notary-backend-api.openapi.json`

- [ ] **Step 1: Implement app-api**

Define Notary access, matters, cases, parties, files, download packages, and events under `/app/v3/api/notary`.

- [ ] **Step 2: Implement backend-api**

Define organization profiles, matter SKU management, case management, assignments, staff view, and reports under `/backend/v3/api/notary`.

- [ ] **Step 3: Verify tests**

Run: `pnpm test`
Expected: OpenAPI assertions pass, SDK/facade assertions still fail until later tasks are complete.

### Task 4: SDK Families And Facades

**Files:**
- Create: `sdks/sdkwork-notary-app-sdk/.sdkwork-assembly.json`
- Create: `sdks/sdkwork-notary-app-sdk/sdk-manifest.json`
- Create: `sdks/sdkwork-notary-app-sdk/specs/component.spec.json`
- Create: `sdks/sdkwork-notary-app-sdk/specs/README.md`
- Create: `sdks/sdkwork-notary-app-sdk/README.md`
- Create: `sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts`
- Create: `sdks/sdkwork-notary-backend-sdk/.sdkwork-assembly.json`
- Create: `sdks/sdkwork-notary-backend-sdk/sdk-manifest.json`
- Create: `sdks/sdkwork-notary-backend-sdk/specs/component.spec.json`
- Create: `sdks/sdkwork-notary-backend-sdk/specs/README.md`
- Create: `sdks/sdkwork-notary-backend-sdk/README.md`
- Create: `sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/composed/index.ts`

- [ ] **Step 1: Implement SDK metadata**

Declare `sdkwork-appbase-*`, `sdkwork-commerce-*`, and `sdkwork-drive-*` as dependency SDKs.

- [ ] **Step 2: Implement composed facades**

Expose high-level Notary orchestration methods that call injected Notary, Drive, Commerce, and Appbase clients.

- [ ] **Step 3: Verify tests**

Run: `pnpm test`
Expected: all tests pass.

### Task 5: Documentation

**Files:**
- Create: `README.md`

- [ ] **Step 1: Document architecture**

Document table ownership, API ownership, SDK dependency boundaries, and Drive folder/file management.

- [ ] **Step 2: Final verification**

Run: `pnpm verify`
Expected: PASS.

