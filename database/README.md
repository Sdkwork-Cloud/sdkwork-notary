# NOTARY Database Module

Canonical lifecycle assets for `sdkwork-notary` per `DATABASE_FRAMEWORK_SPEC.md`.

## Commands

```bash
pnpm run db:materialize:contract
pnpm run db:validate
pnpm run db:plan
pnpm run db:migrate
```

Schema authority:

- Versioned migrations: `database/migrations/{postgres,sqlite}/0001_notary_foundation.{up,down}.sql`
- Baseline DDL: `database/ddl/baseline/{postgres,sqlite}/`

Runtime bootstrap: `sdkwork-notary-database-host` / `bootstrap_notary_database_from_env()`.

Repository integration tests apply the sqlite migration through `schema_migration::notary_foundation_migration_sql()`.
