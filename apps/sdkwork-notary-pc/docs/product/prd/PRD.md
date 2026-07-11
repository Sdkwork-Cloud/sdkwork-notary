# SDKWork Notary PC PRD

Status: active
Owner: SDKWork maintainers
Application: sdkwork-notary-pc
Updated: 2026-07-11
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## 1. Background And Problem

The Notary PC application needs to support both daily notary work and internal operational administration. These surfaces share a renderer and authenticated session, but they have different users, routes, permissions, and SDK contracts. Matter administration was previously absent, leaving operators without a supported way to configure sellable notary services.

## 2. Target Users

- Notary staff using the app workspace to submit and process cases.
- Internal notary administrators and authorized operators managing matter merchandise.
- Support and audit staff with explicit backend permissions.

## 3. Goals And Non-Goals

Goals:

- Preserve the app/user workspace at `/notary` with app SDK boundaries.
- Add a dedicated backend-admin at `/admin` with separate core, shell, and capability packages.
- Provide server-paginated matter search, status filtering, create, edit, activate, and deactivate workflows.
- Make loading, empty, error, permission, validation, and success states explicit.
- Use one authenticated TokenManager without allowing app packages to import backend SDKs.

Non-goals:

- A marketing page or a generic commerce operations suite.
- Payment intent creation from the matter page.
- Hard deletion or arbitrary JSON editing.
- Client-side pagination over a full matter download.

## 4. Scope

- `/notary`: existing app/user Notary workspace.
- `/admin/notary/matters`: internal matter management.
- `pc-admin-core`: backend SDK/provider, runtime config, operator permission context, and audit helpers.
- `pc-admin-shell`: admin layout, navigation, route guard, and route assembly.
- `pc-admin-merchandise`: matter pages, components, hooks, services, i18n, route metadata, and tests.

## 5. User Scenarios

1. An authenticated notary administrator opens the admin matter list and filters by search text or lifecycle status.
2. An operator creates a draft or active matter with title, description, price, currency, and supported typed metadata.
3. An authorized operator edits a matter or changes it between active and inactive states.
4. A read-only operator can inspect matters but cannot see or execute mutation commands.
5. An app user remains in `/notary` and never loads backend SDK behavior through app packages.

## 6. Success Metrics

- Initial list renders one server page and follows `pageInfo.nextCursor` for further pages.
- Zero raw HTTP calls, manual auth headers, generated transport imports, or local API DTO copies in admin packages.
- Zero backend SDK imports from existing app/user core, shell, or feature packages.
- All required PC typecheck, build, package tests, and architecture validators pass.

## 7. Phases

- Phase 1 (completed): matter management and admin package/runtime separation.
- Phase 2 (future): additional operator capabilities added as independent `pc-admin-<capability>` packages after their backend contracts exist.

## 8. Linked Requirements

- Repository requirement: `docs/product/requirements/REQ-2026-0001-notary-pc-admin-matters.md`.

## 9. Open Questions

- None for the current matter-management release. New order/payment operational pages require owner SDK contracts and separate capability packages.
