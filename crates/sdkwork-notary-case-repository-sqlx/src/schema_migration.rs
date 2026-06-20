//! Canonical notary schema SQL sourced from `database/migrations/`.

/// Returns the sqlite foundation migration used by integration tests and local repositories.
pub fn notary_foundation_migration_sql() -> &'static str {
    include_str!("../../../database/migrations/sqlite/0001_notary_foundation.up.sql")
}

/// Returns the postgres foundation migration for host bootstrap helpers.
pub fn notary_foundation_postgres_migration_sql() -> &'static str {
    include_str!("../../../database/migrations/postgres/0001_notary_foundation.up.sql")
}
