# SDKWork Notary PC Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-11
Specs: ARCHITECTURE_DECISION_SPEC.md, DOCUMENTATION_SPEC.md, APP_PC_ARCHITECTURE_SPEC.md, BACKEND_UI_SPEC.md, APP_SDK_INTEGRATION_SPEC.md, FRONTEND_CODE_SPEC.md, SECURITY_SPEC.md

## 1. Architecture Overview

One React renderer hosts distinct app and internal admin route trees:

```text
root bootstrap and AuthGate
  +-- /notary -> pc-shell -> pc-notary -> app SDK through pc-core
  +-- /admin  -> pc-admin-shell -> pc-admin-merchandise -> backend SDK through pc-admin-core
```

The route prefix does not define the security surface. Each admin package declares `component.surface=backend-admin` and `package.json#sdkwork.surface=backend-admin`.

## 2. Technology Choices

- React 19, React Router, Vite, TypeScript strict mode, i18next, and Lucide icons.
- Generated composed SDK imports: `@sdkwork/notary-app-sdk` for app/user and `@sdkwork/notary-backend-sdk` for internal admin.
- Existing appbase auth runtime and one global TokenManager shared through explicit core boundaries.
- Dense operator layout using accessible native controls and package-scoped CSS; no page or service constructs an SDK client.

## 3. System Boundaries And Modules

- `@sdkwork/notary-pc-core`: app SDK/session/runtime only; backend SDKs are forbidden.
- `@sdkwork/notary-pc-shell`: app layout and `/notary` route assembly only.
- `@sdkwork/notary-pc-notary`: app/user case workspace.
- `@sdkwork/notary-pc-admin-core`: Notary backend SDK provider/factory, backend base URL, shared TokenManager binding, operator context, permission checks, and audit metadata.
- `@sdkwork/notary-pc-admin-shell`: admin layout, menu, route guard, and `/admin` route assembly; no business transport.
- `@sdkwork/notary-pc-admin-merchandise`: Matter Management feature; services consume the injected backend client, pages compose hooks/components, and types contain view models only.

## 4. Directory And Package Layout

```text
packages/
  sdkwork-notary-pc-admin-core/
  sdkwork-notary-pc-admin-shell/
  sdkwork-notary-pc-admin-merchandise/
```

Each package owns `package.json`, `src/index.ts`, a focused internal structure, tests, README, and `specs/component.spec.json`. Root `src/` remains limited to bootstrap, AuthGate, and top-level route composition.

## 5. API, SDK, And Data Ownership

- The admin feature calls `notary.matters.management.list`, `notary.matters.create`, and `notary.matters.update` through `@sdkwork/notary-backend-sdk`.
- List queries use `pageSize`, `cursor`, `q`, `organizationId`, and `status` generated options; HTTP serialization remains `page_size` and canonical SDKWork wire names.
- API DTOs come from the backend SDK. The feature owns only form/view state.
- Merchandise remains the SPU/SKU system of record. Order and Payment remain behind backend orchestration and are not direct frontend dependencies.
- No browser persistence is added for matter, order, payment, tokens, or credentials.

## 6. Security, Privacy, And Observability

- AuthGate establishes the authenticated session. Admin core binds the same TokenManager to the backend SDK without implementing login or token storage.
- Admin route guards require the backend-admin user surface and matter read permission. Command buttons separately require create/update permissions.
- Frontend permission checks are advisory; the backend enforces tenant, organization, permission, and audit policy.
- Errors are normalized into operator-safe messages and preserve trace ids when available. Credentials and matter payloads are not logged.

## 7. Deployment And Runtime Topology

The browser derives one public gateway URL from the PC environment/topology adapter. Admin core accepts an explicit backend base URL and fails closed in production when it is absent. Standalone and cloud profiles keep identical SDK and route contracts.

## 8. Architecture Decision Index

- Repository ADR: `docs/architecture/decisions/ADR-20260711-notary-admin-merchandise.md`.

## 9. Verification

- PC typecheck, package tests, and production build.
- Component port, application layering, frontend composition, permission composition, SDK import, pagination, and Tailwind/style integration checks where applicable.
- Browser verification at desktop and narrow viewports for list, form, permission, empty, loading, and error states.
