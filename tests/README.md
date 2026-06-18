# Tests

Cross-cutting verification entrypoints for the notary domain library workspace.

- Contract tests: `sdks/test/*.test.mjs`
- Standard architecture: `scripts/verify-notary-standard-architecture.test.mjs`
- Topology baggage: `scripts/dev/sdkwork-notary-topology-baggage.test.mjs`
- Rust workspace: `cargo test --workspace --target-dir target-codex-test`

Run the full gate with `pnpm verify`.
