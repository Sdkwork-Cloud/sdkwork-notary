# Applications

`sdkwork-notary` is primarily a **contract-first domain library** repository. It owns notary OpenAPI authorities, TypeScript SDK families, Rust route crates, and SQLx persistence.

## H5 application root

The phone-first mobile client lives at `apps/sdkwork-notary-h5/` and follows `sdkwork-specs/APP_H5_ARCHITECTURE_SPEC.md`.

- Root `src/` stays thin: bootstrap, auth gate, route assembly, and mobile shell only.
- Business screens and services live in `packages/sdkwork-notary-h5-*`.
- UI flows call `@sdkwork/notary-app-sdk` composed APIs through `sdkwork-notary-h5-core`.

Run locally from the repository root:

```bash
pnpm dev:browser
```

Or from the H5 app root:

```bash
cd apps/sdkwork-notary-h5
pnpm install
pnpm dev
```

## Host integration

Other host applications (for example IM PC) can still wire `sdkwork-router-notary-*` route crates and consume `sdkwork-notary-*` SDK families directly. Catalog and release metadata for the domain library remain at the repository root in `sdkwork.app.config.json`.
