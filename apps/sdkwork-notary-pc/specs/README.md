# PC Local Specs

Application-level contracts extend the root SDKWork standards without copying them. Every authored package owns its own `specs/component.spec.json`.

Active surface families:

- App/user: `sdkwork-notary-pc-core`, `sdkwork-notary-pc-shell`, `sdkwork-notary-pc-notary`.
- Backend-admin: `sdkwork-notary-pc-admin-core`, `sdkwork-notary-pc-admin-shell`, `sdkwork-notary-pc-admin-merchandise`.

The backend-admin family is the only PC boundary allowed to consume `@sdkwork/notary-backend-sdk`.
