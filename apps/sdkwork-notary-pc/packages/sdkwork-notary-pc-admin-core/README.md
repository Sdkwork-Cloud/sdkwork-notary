# SDKWork Notary PC Admin Core

Backend-admin runtime boundary for the Notary PC application. The package constructs the composed `@sdkwork/notary-backend-sdk` client with the PC application's global TokenManager and exposes operator, permission, and audit context helpers.

Application and user-facing packages must not import this package. Admin capabilities consume it only through the public package exports.

The `./sdk`, `./modules`, `./host`, `./session`, and `./composition` entrypoints
are public composition boundaries over the same runtime. They do not create
additional SDK clients or authentication state.

- Component contract: `specs/component.spec.json`
- Canonical PC architecture: `../../../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`
- Canonical backend UI boundary: `../../../../../sdkwork-specs/BACKEND_UI_SPEC.md`
