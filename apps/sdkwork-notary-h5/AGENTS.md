# SDKWork Notary H5

Phone-first H5 application root for the SDKWork Notary domain library.

- Follow `sdkwork-specs/APP_H5_ARCHITECTURE_SPEC.md`.
- Keep root `src/` thin: bootstrap, providers, route assembly, and mobile shell only.
- Put business screens and services in `packages/sdkwork-notary-h5-*`.
- UI must call services that consume `@sdkwork/notary-app-sdk` composed APIs only.
