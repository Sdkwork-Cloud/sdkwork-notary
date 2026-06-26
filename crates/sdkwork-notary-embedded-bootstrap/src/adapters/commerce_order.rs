use async_trait::async_trait;
use sdkwork_commerce_contract_service::CommerceServiceError;
use sdkwork_commerce_order_repository_sqlx::{
    PostgresCommerceOrderStore, SqliteCommerceOrderStore,
};
use sdkwork_commerce_order_service::{
    CheckoutLineInput, CreateCheckoutQuoteCommand, CreateCheckoutSessionCommand,
    CreateOwnerOrderCommand,
};
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_notary_case_service::{
    CommerceCreateOrderCommand, CommerceMatterCommand, CommerceMatterListQuery,
    CommerceMatterRecord, CommerceMatterUpdateCommand, CommerceOrderReference, CommercePort,
};
use sqlx::Row;

enum OrderStore {
    Sqlite(SqliteCommerceOrderStore),
    Postgres(PostgresCommerceOrderStore),
}

pub struct CommerceOrderPort {
    store: OrderStore,
    query_pool: DatabasePool,
}

impl CommerceOrderPort {
    pub fn new(pool: DatabasePool) -> Self {
        let store = match &pool {
            DatabasePool::Sqlite(sqlite_pool, _) => {
                OrderStore::Sqlite(SqliteCommerceOrderStore::new(sqlite_pool.clone()))
            }
            DatabasePool::Postgres(postgres_pool, _) => {
                OrderStore::Postgres(PostgresCommerceOrderStore::new(postgres_pool.clone()))
            }
        };
        Self {
            store,
            query_pool: pool,
        }
    }
}

#[async_trait]
impl CommercePort for CommerceOrderPort {
    async fn create_notary_order(
        &self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError> {
        let line = CheckoutLineInput::new(command.sku_id.as_str(), 1)
            .map_err(map_commerce_validation)?;
        let request_no = format!("notary-{}", slug(&command.idempotency_key));
        let session_idempotency = format!("{}-session", command.idempotency_key);
        let session_command = CreateCheckoutSessionCommand::new(
            command.tenant_id.as_str(),
            Some(command.organization_id.as_str()),
            command.owner_user_id.as_str(),
            "CNY",
            vec![line],
            request_no.as_str(),
            session_idempotency.as_str(),
        )
        .map_err(map_commerce_validation)?;

        let session = match &self.store {
            OrderStore::Sqlite(store) => store
                .create_checkout_session(session_command)
                .await
                .map_err(map_commerce_error)?,
            OrderStore::Postgres(store) => store
                .create_checkout_session(session_command)
                .await
                .map_err(map_commerce_error)?,
        };

        let quote_command = CreateCheckoutQuoteCommand::new(
            command.tenant_id.as_str(),
            Some(command.organization_id.as_str()),
            command.owner_user_id.as_str(),
            session.checkout_session_id.as_str(),
            request_no.as_str(),
            format!("{}-quote", command.idempotency_key).as_str(),
        )
        .map_err(map_commerce_validation)?;

        match &self.store {
            OrderStore::Sqlite(store) => {
                store
                    .create_checkout_quote(quote_command)
                    .await
                    .map_err(map_commerce_error)?;
            }
            OrderStore::Postgres(store) => {
                store
                    .create_checkout_quote(quote_command)
                    .await
                    .map_err(map_commerce_error)?;
            }
        }

        let order_command = CreateOwnerOrderCommand::new(
            command.tenant_id.as_str(),
            Some(command.organization_id.as_str()),
            command.owner_user_id.as_str(),
            session.checkout_session_id.as_str(),
            request_no.as_str(),
            command.idempotency_key.as_str(),
        )
        .map_err(map_commerce_validation)?;

        let outcome = match &self.store {
            OrderStore::Sqlite(store) => store
                .create_owner_order(order_command)
                .await
                .map_err(map_commerce_error)?,
            OrderStore::Postgres(store) => store
                .create_owner_order(order_command)
                .await
                .map_err(map_commerce_error)?,
        };

        let (order_item_id, currency_code) =
            load_order_item(&self.query_pool, &command.tenant_id, &outcome.order_id).await?;

        Ok(CommerceOrderReference {
            order_id: outcome.order_id,
            order_item_id,
            sku_id: command.sku_id,
            matter_title: command.title,
            fee_amount: outcome.total_amount.as_str().to_owned(),
            currency_code,
        })
    }

    async fn cancel_notary_order(&self, order_id: &str) -> Result<(), NotaryServiceError> {
        let _ = order_id;
        Err(NotaryServiceError::provider_unavailable(
            "embedded commerce cancel_notary_order is not configured",
        ))
    }

    async fn list_notary_matters(
        &self,
        query: CommerceMatterListQuery,
    ) -> Result<Vec<CommerceMatterRecord>, NotaryServiceError> {
        let _ = query;
        Ok(Vec::new())
    }

    async fn create_notary_matter(
        &self,
        _command: CommerceMatterCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "embedded commerce matter creation requires split commerce admin API",
        ))
    }

    async fn update_notary_matter(
        &self,
        _command: CommerceMatterUpdateCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "embedded commerce matter update requires split commerce admin API",
        ))
    }
}

async fn load_order_item(
    pool: &DatabasePool,
    tenant_id: &str,
    order_id: &str,
) -> Result<(String, String), NotaryServiceError> {
    match pool {
        DatabasePool::Sqlite(sqlite_pool, _) => {
            let row = sqlx::query(
                "SELECT id, 'CNY' AS currency_code \
                 FROM commerce_order_item WHERE tenant_id = ?1 AND order_id = ?2 LIMIT 1",
            )
            .bind(tenant_id)
            .bind(order_id)
            .fetch_optional(sqlite_pool)
            .await
            .map_err(storage_error)?
            .ok_or_else(|| {
                NotaryServiceError::not_found(
                    "commerce order item was not created for notary case",
                )
            })?;
            Ok((
                row.try_get("id").map_err(storage_error)?,
                row.try_get("currency_code").map_err(storage_error)?,
            ))
        }
        DatabasePool::Postgres(postgres_pool, _) => {
            let row = sqlx::query(
                "SELECT id, 'CNY' AS currency_code \
                 FROM commerce_order_item WHERE tenant_id = $1 AND order_id = $2 LIMIT 1",
            )
            .bind(tenant_id)
            .bind(order_id)
            .fetch_optional(postgres_pool)
            .await
            .map_err(storage_error)?
            .ok_or_else(|| {
                NotaryServiceError::not_found(
                    "commerce order item was not created for notary case",
                )
            })?;
            Ok((
                row.try_get("id").map_err(storage_error)?,
                row.try_get("currency_code").map_err(storage_error)?,
            ))
        }
    }
}

fn slug(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .chars()
        .take(24)
        .collect()
}

fn map_commerce_validation(error: CommerceServiceError) -> NotaryServiceError {
    NotaryServiceError::validation(error.message())
}

fn map_commerce_error(error: CommerceServiceError) -> NotaryServiceError {
    NotaryServiceError::storage(error.message())
}

fn storage_error(error: sqlx::Error) -> NotaryServiceError {
    NotaryServiceError::storage(error.to_string())
}
