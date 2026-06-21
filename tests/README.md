# Tests

Cross-cutting verification index for `sdkwork-notary`.

## Executable suites

- `sdks/test/*.test.mjs` — OpenAPI, SDK, web-framework, database, and integration contract tests
- `scripts/verify-notary-standard-architecture.test.mjs` — workspace and framework alignment
- `scripts/verify-notary-utils-standard.test.mjs` — `sdkwork-utils-rust` integration
- `scripts/dev/sdkwork-notary-topology-baggage.test.mjs` — retired topology vocabulary scan
- `apps/sdkwork-notary-h5/src/__tests__/*.test.mjs` — H5 architecture contracts
- `tests/contract/*.test.mjs` — manifest, workflow, SDK generation, and H5 component specs

## Directories

- `contract/` — standards and manifest contract tests
- `integration/` — host wiring and runtime integration checks
- `e2e/` — end-to-end verification flows
- `fixtures/` — shared fixtures
- `static/` — static assets for verification

Run the merge gate with:

```bash
pnpm verify
```
