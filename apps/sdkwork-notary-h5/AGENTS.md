# SDKWork Notary H5

Phone-first H5 application root for the SDKWork Notary domain library.

## SDKWORK Soul

Read `../../../sdkwork-specs/SOUL.md` before executing tasks in this root.

## Application Identity

- App key: `sdkwork-notary-h5`
- Manifest: `sdkwork.app.config.json`
- Domain library owner: repository root `sdkwork-notary`

## Required Specs

- `../../../sdkwork-specs/APP_H5_ARCHITECTURE_SPEC.md`
- `../../../sdkwork-specs/APP_MOBILE_REACT_UI_SPEC.md`
- `../../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md`
- `../../../sdkwork-specs/IAM_LOGIN_INTEGRATION_SPEC.md`
- `../../../sdkwork-specs/CONFIG_SPEC.md`
- `../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md`
- `../../../sdkwork-specs/TEST_SPEC.md`

## Architecture Rules

- Root `src/` stays thin: bootstrap, AuthGate, route assembly, and mobile shell only.
- Business screens and services live in `packages/sdkwork-notary-h5-*`.
- UI flows call `@sdkwork/notary-app-sdk` composed APIs through `sdkwork-notary-h5-core`.
- TokenManager uses `@sdkwork/sdk-common/createTokenManager`; appbase IAM H5 login UI mounts in `AuthGate` when available.

## Build, Test, and Verification

```bash
pnpm dev
pnpm typecheck
pnpm test
pnpm build
```

From repository root:

```bash
pnpm dev:browser
pnpm test:browser
pnpm build:browser
```

## Agent Execution Rules

Keep SDK construction in bootstrap/core packages only. Do not add raw HTTP, manual auth headers, or backend SDK usage in app packages.

## Human Review Rules

Request human review before changing auth behavior, manifest release metadata, Capacitor host boundaries, or public package exports.
