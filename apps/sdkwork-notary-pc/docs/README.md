# SDKWork Notary PC Documentation

The PC root ships two separately governed surfaces in one renderer:

- App/user workspace under `/notary`, using `@sdkwork/notary-app-sdk`.
- Internal backend-admin under `/admin`, using `@sdkwork/notary-backend-sdk` only through `@sdkwork/notary-pc-admin-core`.

Canon documents:

- [Product PRD](product/prd/PRD.md)
- [Technical architecture](architecture/tech/TECH_ARCHITECTURE.md)

Module contracts live in each package's `specs/component.spec.json`. This directory is narrative and discovery only.
