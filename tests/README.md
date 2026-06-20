# Cross-cutting verification

Contract and integration tests live under:

- `sdks/test/` — OpenAPI, SDK, runtime, database framework, and storage contract tests
- `scripts/verify-notary-standard-architecture.test.mjs` — workspace dictionary and framework alignment
- `scripts/verify-notary-utils-standard.test.mjs` — sdkwork-utils integration checks
- `scripts/dev/` — topology baggage and dev helper tests

Run the aggregate gate with `pnpm verify`.
