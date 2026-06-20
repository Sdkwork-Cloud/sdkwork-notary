# NOTARY Database Module

Canonical lifecycle assets for `sdkwork-notary` per `DATABASE_FRAMEWORK_SPEC.md`.

## Commands

```bash
pnpm run db:materialize:contract
pnpm run db:validate
```

Legacy SQL: `crates/sdkwork-notary-case-repository-sqlx/migrations/0001_notary_foundation.sql` → `database/ddl/baseline/postgres/0001_notary_legacy_baseline.sql`

Runtime bootstrap: `sdkwork-notary-database-host` / `connect_and_bootstrap_notary_database_from_env()`.
