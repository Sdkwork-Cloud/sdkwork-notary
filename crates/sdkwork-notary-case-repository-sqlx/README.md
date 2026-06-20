# sdkwork-notary-case-repository-sqlx

SQLx-backed notary case repository for local and private deployments.

- SQLite implementation: `SqliteNotaryCaseRepository`
- PostgreSQL implementation: `PostgresNotaryCaseRepository`
- Schema authority: `database/migrations/{postgres,sqlite}/0001_notary_foundation.up.sql`
- Lifecycle bootstrap: `sdkwork-notary-database-host` via `connect_and_bootstrap_notary_database_from_env()`
- PII vault: AES-256-GCM field encryption via `NOTARY_PII_VAULT_KEY`

## Verify

```bash
pnpm db:validate
cargo test -p sdkwork-notary-case-repository-sqlx --target-dir target-codex-test
```

Set `SDKWORK_NOTARY_DATABASE_URL` to a PostgreSQL connection string when running live PostgreSQL verification in host applications.
