use async_trait::async_trait;
use sdkwork_contract_service::CommerceServiceError;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_notary_case_service::{
    CommerceCreateOrderCommand, CommerceMatterCommand, CommerceMatterListQuery,
    CommerceMatterRecord, CommerceMatterUpdateCommand, CommerceOrderReference, CommercePort,
};
use sdkwork_order_repository_sqlx::{PostgresCommerceOrderStore, SqliteCommerceOrderStore};
use sdkwork_order_service::{
    CheckoutLineInput, CreateCheckoutQuoteCommand, CreateCheckoutSessionCommand,
    CreateOwnerOrderCommand,
};
use serde_json::Value;
use sqlx::Row;

enum OrderStore {
    Sqlite(SqliteCommerceOrderStore),
    Postgres(PostgresCommerceOrderStore),
}

pub struct CommerceOrderPort {
    store: OrderStore,
    query_pool: DatabasePool,
    tenant_id: String,
    owner_user_id: String,
}

impl CommerceOrderPort {
    pub fn new(
        pool: DatabasePool,
        tenant_id: impl Into<String>,
        owner_user_id: impl Into<String>,
    ) -> Self {
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
            tenant_id: tenant_id.into(),
            owner_user_id: owner_user_id.into(),
        }
    }
}

#[async_trait]
impl CommercePort for CommerceOrderPort {
    async fn create_notary_order(
        &self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError> {
        let line =
            CheckoutLineInput::new(command.sku_id.as_str(), 1).map_err(map_commerce_validation)?;
        let request_no = format!("notary-{}", slug(&command.idempotency_key));
        let session_idempotency = format!("{}-session", command.idempotency_key);
        let session_command = CreateCheckoutSessionCommand::new(
            self.tenant_id.as_str(),
            Some(command.organization_id.as_str()),
            self.owner_user_id.as_str(),
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
            self.tenant_id.as_str(),
            Some(command.organization_id.as_str()),
            self.owner_user_id.as_str(),
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
            self.tenant_id.as_str(),
            Some(command.organization_id.as_str()),
            self.owner_user_id.as_str(),
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
            load_order_item(&self.query_pool, self.tenant_id.as_str(), &outcome.order_id).await?;

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
        let limit = query.page_size.max(0);
        let offset = query.offset.max(0);
        match &self.query_pool {
            DatabasePool::Sqlite(pool, _) => {
                list_notary_matters_sqlite(
                    pool,
                    self.tenant_id.as_str(),
                    query.search_term.as_deref(),
                    query.status.as_deref(),
                    limit,
                    offset,
                )
                .await
            }
            DatabasePool::Postgres(pool, _) => {
                list_notary_matters_postgres(
                    pool,
                    self.tenant_id.as_str(),
                    query.search_term.as_deref(),
                    query.status.as_deref(),
                    limit,
                    offset,
                )
                .await
            }
        }
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

async fn list_notary_matters_sqlite(
    pool: &sqlx::SqlitePool,
    tenant_id: &str,
    search_term: Option<&str>,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommerceMatterRecord>, NotaryServiceError> {
    let rows = sqlx::query(
        r#"
        SELECT id,
               COALESCE(spu_id, '') AS spu_id,
               COALESCE(sku_no, id) AS sku_no,
               COALESCE(NULLIF(title, ''), name, id) AS title,
               CAST(price_amount AS TEXT) AS price_amount,
               CAST(COALESCE(original_price_amount, price_amount) AS TEXT) AS original_price_amount,
               COALESCE(currency_code, 'CNY') AS currency_code,
               COALESCE(status, 'active') AS status,
               COALESCE(spec_json, '{}') AS spec_json
        FROM commerce_product_sku
        WHERE tenant_id = ?1
          AND LOWER(COALESCE(fulfillment_type, '')) = 'notary'
          AND (?2 IS NULL OR LOWER(COALESCE(status, '')) = LOWER(?2))
          AND (
              ?3 IS NULL
              OR LOWER(COALESCE(NULLIF(title, ''), name, id)) LIKE '%' || LOWER(?3) || '%'
          )
        ORDER BY updated_at DESC, id DESC
        LIMIT ?4 OFFSET ?5
        "#,
    )
    .bind(tenant_id)
    .bind(status)
    .bind(search_term)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(storage_error)?;

    rows.iter().map(matter_record_from_sqlite_row).collect()
}

async fn list_notary_matters_postgres(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    search_term: Option<&str>,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommerceMatterRecord>, NotaryServiceError> {
    let rows = sqlx::query(
        r#"
        SELECT id,
               COALESCE(spu_id, '') AS spu_id,
               COALESCE(sku_no, id) AS sku_no,
               COALESCE(NULLIF(title, ''), name, id) AS title,
               CAST(price_amount AS TEXT) AS price_amount,
               CAST(COALESCE(original_price_amount, price_amount) AS TEXT) AS original_price_amount,
               COALESCE(currency_code, 'CNY') AS currency_code,
               COALESCE(status, 'active') AS status,
               COALESCE(spec_json, '{}') AS spec_json
        FROM commerce_product_sku
        WHERE tenant_id = $1
          AND LOWER(COALESCE(fulfillment_type, '')) = 'notary'
          AND ($2::text IS NULL OR LOWER(COALESCE(status, '')) = LOWER($2))
          AND (
              $3::text IS NULL
              OR LOWER(COALESCE(NULLIF(title, ''), name, id)) LIKE '%' || LOWER($3) || '%'
          )
        ORDER BY updated_at DESC, id DESC
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(tenant_id)
    .bind(status)
    .bind(search_term)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(storage_error)?;

    rows.iter().map(matter_record_from_postgres_row).collect()
}

fn matter_record_from_sqlite_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<CommerceMatterRecord, NotaryServiceError> {
    let spec_raw: String = row.try_get("spec_json").map_err(storage_error)?;
    let spec =
        serde_json::from_str(&spec_raw).unwrap_or_else(|_| Value::Object(Default::default()));
    Ok(CommerceMatterRecord {
        sku_id: row.try_get("id").map_err(storage_error)?,
        spu_id: row.try_get("spu_id").map_err(storage_error)?,
        sku_no: row.try_get("sku_no").map_err(storage_error)?,
        title: row.try_get("title").map_err(storage_error)?,
        description: None,
        price_amount: row.try_get("price_amount").map_err(storage_error)?,
        original_price_amount: Some(
            row.try_get("original_price_amount")
                .map_err(storage_error)?,
        ),
        currency_code: row.try_get("currency_code").map_err(storage_error)?,
        status: row.try_get("status").map_err(storage_error)?,
        spec,
    })
}

fn matter_record_from_postgres_row(
    row: &sqlx::postgres::PgRow,
) -> Result<CommerceMatterRecord, NotaryServiceError> {
    let spec_raw: String = row.try_get("spec_json").map_err(storage_error)?;
    let spec =
        serde_json::from_str(&spec_raw).unwrap_or_else(|_| Value::Object(Default::default()));
    Ok(CommerceMatterRecord {
        sku_id: row.try_get("id").map_err(storage_error)?,
        spu_id: row.try_get("spu_id").map_err(storage_error)?,
        sku_no: row.try_get("sku_no").map_err(storage_error)?,
        title: row.try_get("title").map_err(storage_error)?,
        description: None,
        price_amount: row.try_get("price_amount").map_err(storage_error)?,
        original_price_amount: Some(
            row.try_get("original_price_amount")
                .map_err(storage_error)?,
        ),
        currency_code: row.try_get("currency_code").map_err(storage_error)?,
        status: row.try_get("status").map_err(storage_error)?,
        spec,
    })
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
                NotaryServiceError::not_found("commerce order item was not created for notary case")
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
                NotaryServiceError::not_found("commerce order item was not created for notary case")
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
