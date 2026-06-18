# sdkwork-notary-case-repository-sqlx

SQLx-backed notary case repository for local and private deployments.

- SQLite implementation: `SqliteNotaryCaseRepository`
- PostgreSQL implementation: `PostgresNotaryCaseRepository`
- Migration contract: `migrations/0001_notary_foundation.sql`
- PII vault: AES-256-GCM field encryption via `NOTARY_PII_VAULT_KEY`

## Verify

```bash
cargo test -p sdkwork-notary-case-repository-sqlx --target-dir target-codex-test
```

Set `SDKWORK_NOTARY_DATABASE_URL` to a PostgreSQL connection string when running live PostgreSQL verification in host applications.
