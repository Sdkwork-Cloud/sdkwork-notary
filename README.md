# SDKWork Notary

`sdkwork-notary` defines the professional notary business contract layer for SDKWork. The current implementation is contract-first and runtime-ready: database schema, OpenAPI authority documents, SDK family metadata, TypeScript package roots, composed facades, and Rust service orchestration crates are in place so application and backend implementations can share stable boundaries.

## Ownership Model

Notary owns only notary-specific facts:

- `notary_organization_profile`: enables notary business for an enterprise-verified organization and stores its `space_type='notary'` Drive space.
- `notary_case`: one notary case per Commerce order item, with `order_id`, `order_item_id`, `sku_id`, `drive_space_id`, and `drive_folder_node_id`.
- `notary_party`: parties for a case, including encrypted identity and contact fields plus logical order/SKU references.
- `notary_case_assignment`: case assignment facts for IAM organization members.
- `notary_case_event`: timeline and process events.

The system deliberately does not create `notary_document`, `notary_order`, `notary_order_item`, `notary_matter`, `notary_staff_profile`, or `notary_folder` tables.

## Reused Capabilities

- Staff: Appbase/IAM `iam_organization_membership`, roles, positions, and departments.
- Notary matters: Commerce `commerce_product_spu` and `commerce_product_sku`; one notary product maps to one SKU.
- Orders: Commerce `commerce_order` and `commerce_order_item`; every notary case is an order item.
- Files: Drive `dr_drive_space` and `dr_drive_node`; every case stores one `drive_folder_node_id` under `space_type='notary'`.

Notary tables keep performance-critical logical references, but dependency-owned tables remain dependency-owned and are validated through service/SDK boundaries rather than physical cross-service foreign keys.

## API Contracts

- App authored contract: `apis/app-api/notary/notary-app-api.openapi.json`
- Backend authored contract: `apis/backend-api/notary/notary-backend-api.openapi.json`
- App API authority: `generated/openapi/notary-app-api.openapi.json`
- Backend API authority: `generated/openapi/notary-backend-api.openapi.json`
- App prefix: `/app/v3/api/notary`
- Backend prefix: `/backend/v3/api/notary`

All operations are dual-token protected and carry SDKWork owner, authority, permission, tenant scope, data scope, and audit metadata.

## SDKs

- App SDK family: `sdks/sdkwork-notary-app-sdk`
- Backend SDK family: `sdks/sdkwork-notary-backend-sdk`

Dependency SDKs are declared explicitly:

- App: `sdkwork-iam-app-sdk`, `sdkwork-commerce-app-sdk`, `sdkwork-drive-app-sdk`
- Backend: `sdkwork-iam-backend-sdk`, `sdkwork-commerce-backend-sdk`, `sdkwork-drive-backend-sdk`

The TypeScript package roots and composed facades are authored outside `generated/server-openapi`:

- `sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts` as `@sdkwork/notary-app-sdk`
- `sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts`
- `sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/src/index.ts` as `@sdkwork/notary-backend-sdk`
- `sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/composed/index.ts`

These package roots export generated clients, generated types, and approved composed facades. The facades use injected SDK clients only. They do not construct raw HTTP clients or manual credential headers.

## Runtime Crates

- `crates/sdkwork-notary-case-contract`: shared domain records, commands, status values, typed service errors, runtime context, and service contract metadata.
- `crates/sdkwork-notary-case-service`: orchestration service over explicit Appbase, Commerce, Drive, and Notary repository ports.
- `crates/sdkwork-notary-case-repository-sqlx`: SQLx-backed Notary repository implementations (`SqliteNotaryCaseRepository`, `PostgresNotaryCaseRepository`) with schema sourced from `database/migrations/`.
- `crates/sdkwork-notary-database-host`: database lifecycle bootstrap via `sdkwork-database-lifecycle` SPI.

The runtime layer implements the main notary workflow without owning dependency facts:

- app access retrieval checks the current Appbase Organization member, enterprise verification, notary enablement, organization profile status, and `drive_space_type='notary'` before returning menu visibility and notary permissions;
- opening notary business validates Appbase Organization membership, enterprise verification, notary enablement, and role/position authority before creating the Drive `space_type='notary'` space and upserting `notary_organization_profile`;
- notary matter listing and backend matter management delegate to Commerce SKU/SPU ports; one notary product is represented as one SKU and no local notary matter table is created;
- creating a notary case validates the notary staff member, creates a Commerce order/order item from a notary `sku_id`, creates the Drive case folder, persists `notary_case`, persists parties, and appends the submitted timeline event;
- case updates, status commands, party add/update/delete, party signature attachment, file registration, download package requests, and timeline events are exposed through the app runtime dispatcher;
- backend organization profile update, case management list/retrieve, staff list, assignment create/release, and case summary operations are exposed through the backend runtime dispatcher;
- listing case files loads the case and calls Drive by denormalized `drive_space_type`, `drive_space_id`, and `drive_folder_node_id`, avoiding cross-domain online joins.

Route runtime services are implemented for both app and backend route crates, so the generated route layers can dispatch directly to `sdkwork-notary-case-service` ports:

- `crates/sdkwork-router-notary-app-api`
- `crates/sdkwork-router-notary-backend-api`

## Chat PC Integration

The frontend notary capability package lives in this repository:

- `apps/sdkwork-notary-pc`
- `packages/sdkwork-notary-pc-notary/src/services/NotaryService.ts`

SDKWork IM PC integrates the package directly:

- `../sdkwork-im/apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/notaryPcIntegration.ts`
- `../sdkwork-im/apps/sdkwork-im-pc/src/bootstrap/notaryPc.ts`

The IM app root includes `@sdkwork/notary-app-sdk` and `@sdkwork/notary-pc-notary` as workspace dependencies. IM core constructs the generated Notary App SDK client with the shared TokenManager, while `sdkwork-notary-pc-notary` owns the UI and service facade over `createNotaryApi`.

The obsolete `integrations/sdkwork-chat-pc/` fork and `sdkwork-im-pc-notary` package were removed. Contract tests `sdks/test/notary-chat-pc-real-app-integration.test.mjs` and `apps/sdkwork-notary-pc/src/__tests__/pc-architecture.contract.test.mjs` verify the notary-owned PC package, IM integration wiring, and absence of duplicate notary forks.

Standalone PC client commands from the repository root:

```powershell
pnpm dev:desktop
pnpm test:desktop
pnpm build:desktop
```

## Topology

Runtime topology follows SDKWork v2 (`application-http-gateway`):

- Spec: `specs/topology.spec.json`
- Profiles: `configs/topology/*.env`
- Adapter: `scripts/lib/notary-topology.mjs`
- Dev entry: `pnpm dev`

See `docs/topology-standard.md` and `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md`.

## Verification

Run from this repository root:

```powershell
pnpm dev
pnpm verify
pnpm api:materialize
pnpm api:materialize
pnpm sdk:generate
```

Or run the individual gates:

```powershell
pnpm test:topology-validate
pnpm test:topology-baggage
pnpm db:validate
pnpm test:contracts
pnpm format:check
pnpm test:rust
```

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

