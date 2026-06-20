//! SDKWork Notary database pool bootstrap via `sdkwork-database`.

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool, PoolError};

pub use sdkwork_notary_database_host::{
    bootstrap_notary_database, bootstrap_notary_database_from_env, NotaryDatabaseHost,
};

pub type NotaryDatabasePool = DatabasePool;

pub async fn connect_notary_database_pool_from_env() -> Result<NotaryDatabasePool, PoolError> {
    let config = DatabaseConfig::from_env("NOTARY")?;
    create_pool_from_config(config).await
}

pub async fn connect_and_bootstrap_notary_database_from_env() -> Result<NotaryDatabaseHost, String>
{
    let pool = connect_notary_database_pool_from_env()
        .await
        .map_err(|error| error.to_string())?;
    bootstrap_notary_database(pool).await
}
