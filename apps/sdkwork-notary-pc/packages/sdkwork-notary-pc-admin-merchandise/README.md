# SDKWork Notary PC Admin Merchandise

Backend-admin capability package for the UI labeled "Matter management". Each matter is the Notary projection of a merchandise SPU/SKU and is managed only through the composed Notary backend SDK supplied by `@sdkwork/notary-pc-admin-core`.

The package owns server-paginated list/search/status workflows, create/edit forms, status changes, permissions, route metadata, and package-local operator i18n. It does not own database definitions, raw HTTP, Order or Payment transports, hard deletion, or arbitrary merchandise specification editing.

- Component contract: `specs/component.spec.json`
- Canonical backend UI rules: `../../../../../sdkwork-specs/BACKEND_UI_SPEC.md`
- Canonical pagination rules: `../../../../../sdkwork-specs/PAGINATION_SPEC.md`
