# sdkwork-notary-database-host

Rust database lifecycle bootstrap for the notary domain module.

- Loads `database/database.manifest.json` through `sdkwork-database-spi`
- Runs init and optional auto-migrate via `sdkwork-database-lifecycle`
- Resolves connection config from `SDKWORK_NOTARY_*` environment keys

## Entrypoints

- `bootstrap_notary_database_from_env()` — connect and bootstrap from environment
- `bootstrap_notary_database(pool)` — bootstrap an existing pool

## Verify

```bash
pnpm db:validate
cargo test -p sdkwork-notary-database-host --target-dir target-codex-test
```
