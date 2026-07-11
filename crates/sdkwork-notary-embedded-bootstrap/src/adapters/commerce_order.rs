use std::sync::Arc;

use async_trait::async_trait;
use sdkwork_contract_service::CommerceServiceError;
use sdkwork_database_id::IdGenerator;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_merchandise_repository_sqlx::SqlxSingleSkuMerchandiseRepository;
use sdkwork_merchandise_service::{
    description, public_spec, CreateSingleSkuMerchandiseCommand, SingleSkuMerchandiseListQuery,
    SingleSkuMerchandiseService, SkuRecord, UpdateSingleSkuMerchandiseCommand,
};
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_notary_case_service::{
    CommerceCreateOrderCommand, CommerceMatterCommand, CommerceMatterListPage,
    CommerceMatterListQuery, CommerceMatterRecord, CommerceMatterUpdateCommand,
    CommerceOrderFulfillmentState, CommerceOrderReference, CommercePort,
};
use sdkwork_order_repository_sqlx::{PostgresCommerceOrderStore, SqliteCommerceOrderStore};
use sdkwork_order_service::{
    CancelOwnerOrderCommand, CheckoutLineInput, CreateCheckoutQuoteCommand,
    CreateCheckoutSessionCommand, CreateOwnerOrderCommand, OrderOwnerDetailQuery,
};
use sdkwork_utils_rust::{minor_unit_exponent, sha256_hash};

const NOTARY_FULFILLMENT_TYPE: &str = "notary";
const NOTARY_PRODUCT_TYPE: &str = "notary";
const NOTARY_ORDER_CANCEL_REASON: &str = "notary case creation compensation";

enum OrderStore {
    Sqlite(SqliteCommerceOrderStore),
    Postgres(PostgresCommerceOrderStore),
}

pub struct CommerceOrderPort {
    store: OrderStore,
    merchandise: SingleSkuMerchandiseService<SqlxSingleSkuMerchandiseRepository>,
    tenant_id: String,
    owner_user_id: String,
}

impl CommerceOrderPort {
    pub fn new(
        pool: DatabasePool,
        merchandise_id_generator: Arc<dyn IdGenerator>,
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
        let merchandise = SingleSkuMerchandiseService::new(
            SqlxSingleSkuMerchandiseRepository::new(pool, merchandise_id_generator),
        );
        Self {
            store,
            merchandise,
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
            CheckoutLineInput::new(command.sku_id.as_str(), 1).map_err(map_commerce_error)?;
        let request_digest = sha256_hash(command.idempotency_key.as_bytes());
        let request_no = format!("notary-{}", &request_digest[..24]);
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
        .map_err(map_commerce_error)?;

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
        .map_err(map_commerce_error)?;

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
        .map_err(map_commerce_error)?;

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

        let detail_query = OrderOwnerDetailQuery::new(
            self.tenant_id.as_str(),
            Some(command.organization_id.as_str()),
            self.owner_user_id.as_str(),
            outcome.order_id.as_str(),
        )
        .map_err(map_commerce_error)?;
        let detail = match &self.store {
            OrderStore::Sqlite(store) => store
                .retrieve_owner_order(detail_query)
                .await
                .map_err(map_commerce_error)?,
            OrderStore::Postgres(store) => store
                .retrieve_owner_order(detail_query)
                .await
                .map_err(map_commerce_error)?,
        }
        .ok_or_else(|| {
            NotaryServiceError::not_found("order detail was not created for notary case")
        })?;
        let order_item = match detail.items.as_slice() {
            [item] => item,
            [] => {
                return Err(NotaryServiceError::not_found(
                    "order item was not created for notary case",
                ));
            }
            _ => {
                return Err(NotaryServiceError::conflict(
                    "notary checkout must create exactly one order item",
                ));
            }
        };

        Ok(CommerceOrderReference {
            order_id: outcome.order_id,
            order_item_id: order_item.id.clone(),
            sku_id: command.sku_id,
            matter_title: command.title,
            fee_amount: minor_units_to_major_amount(
                outcome.total_amount.as_str(),
                detail.summary.currency_code.as_str(),
                "order total amount",
            )?,
            currency_code: detail.summary.currency_code,
        })
    }

    async fn cancel_notary_order(
        &self,
        organization_id: &str,
        order_id: &str,
    ) -> Result<(), NotaryServiceError> {
        let command = CancelOwnerOrderCommand::new(
            self.tenant_id.as_str(),
            Some(organization_id),
            self.owner_user_id.as_str(),
            order_id,
            Some(NOTARY_ORDER_CANCEL_REASON),
        )
        .map_err(map_commerce_error)?;
        match &self.store {
            OrderStore::Sqlite(store) => store
                .cancel_owner_order(command)
                .await
                .map_err(map_commerce_error),
            OrderStore::Postgres(store) => store
                .cancel_owner_order(command)
                .await
                .map_err(map_commerce_error),
        }
    }

    async fn get_notary_order_fulfillment_state(
        &self,
        organization_id: &str,
        order_id: &str,
    ) -> Result<CommerceOrderFulfillmentState, NotaryServiceError> {
        let query = OrderOwnerDetailQuery::new(
            self.tenant_id.as_str(),
            Some(organization_id),
            self.owner_user_id.as_str(),
            order_id,
        )
        .map_err(map_commerce_error)?;
        let detail = match &self.store {
            OrderStore::Sqlite(store) => store
                .retrieve_owner_order(query)
                .await
                .map_err(map_commerce_error)?,
            OrderStore::Postgres(store) => store
                .retrieve_owner_order(query)
                .await
                .map_err(map_commerce_error)?,
        }
        .ok_or_else(|| {
            NotaryServiceError::not_found(
                "commerce order was not found for the notary organization",
            )
        })?;
        let payable_amount = detail.summary.total_amount.as_str().to_owned();
        Ok(CommerceOrderFulfillmentState {
            order_id: detail.summary.order_id,
            order_status: detail.summary.status,
            payment_status: detail.payment_status,
            payable_amount,
        })
    }

    async fn list_notary_matters(
        &self,
        query: CommerceMatterListQuery,
    ) -> Result<CommerceMatterListPage, NotaryServiceError> {
        let owner_query = SingleSkuMerchandiseListQuery::new(
            self.tenant_id.as_str(),
            query.organization_id.as_deref(),
            NOTARY_FULFILLMENT_TYPE,
            query.search_term.as_deref(),
            query.status.as_deref(),
            query.page_size,
            query.offset,
        )
        .map_err(map_commerce_error)?;
        let page = self
            .merchandise
            .list(owner_query)
            .await
            .map_err(map_commerce_error)?;
        Ok(CommerceMatterListPage {
            items: page
                .items
                .iter()
                .map(matter_record_from_sku)
                .collect::<Result<Vec<_>, _>>()?,
            has_more: page.has_more,
        })
    }

    async fn create_notary_matter(
        &self,
        command: CommerceMatterCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        let organization_id = command.organization_id.as_deref().ok_or_else(|| {
            NotaryServiceError::validation("organizationId is required for matter creation")
        })?;
        let price_amount = major_amount_to_minor_units(
            command.price_amount.as_str(),
            command.currency_code.as_str(),
            "priceAmount",
        )?;
        let original_price_amount = command
            .original_price_amount
            .as_deref()
            .map(|amount| {
                major_amount_to_minor_units(
                    amount,
                    command.currency_code.as_str(),
                    "originalPriceAmount",
                )
            })
            .transpose()?;
        validate_comparison_price(price_amount.as_str(), original_price_amount.as_deref())?;
        let owner_command = CreateSingleSkuMerchandiseCommand::new(
            self.tenant_id.as_str(),
            organization_id,
            NOTARY_PRODUCT_TYPE,
            NOTARY_FULFILLMENT_TYPE,
            command.title.as_str(),
            command.description.as_deref(),
            price_amount.as_str(),
            original_price_amount.as_deref(),
            command.currency_code.as_str(),
            command.status.as_str(),
            command.spec,
            command.idempotency_key.as_str(),
        )
        .map_err(map_commerce_error)?;
        let record = self
            .merchandise
            .create(owner_command)
            .await
            .map_err(map_commerce_error)?;
        matter_record_from_sku(&record)
    }

    async fn update_notary_matter(
        &self,
        command: CommerceMatterUpdateCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        let price_amount = command
            .price_amount
            .as_deref()
            .map(|amount| {
                let currency_code = command.currency_code.as_deref().ok_or_else(|| {
                    NotaryServiceError::validation(
                        "currencyCode is required when updating priceAmount",
                    )
                })?;
                major_amount_to_minor_units(amount, currency_code, "priceAmount")
            })
            .transpose()?;
        let original_price_amount = match command.original_price_amount.as_ref() {
            None => None,
            Some(None) => Some(None),
            Some(Some(amount)) => {
                let currency_code = command.currency_code.as_deref().ok_or_else(|| {
                    NotaryServiceError::validation(
                        "currencyCode is required when updating originalPriceAmount",
                    )
                })?;
                Some(Some(major_amount_to_minor_units(
                    amount,
                    currency_code,
                    "originalPriceAmount",
                )?))
            }
        };
        if let Some(price_amount) = price_amount.as_deref() {
            validate_comparison_price(
                price_amount,
                original_price_amount
                    .as_ref()
                    .and_then(|amount| amount.as_deref()),
            )?;
        }
        let owner_command = UpdateSingleSkuMerchandiseCommand::new(
            self.tenant_id.as_str(),
            command.organization_id.as_str(),
            command.sku_id.as_str(),
            NOTARY_FULFILLMENT_TYPE,
            Some(NOTARY_PRODUCT_TYPE),
            command.title.as_deref(),
            command
                .description
                .as_ref()
                .map(|description| description.as_deref()),
            price_amount.as_deref(),
            original_price_amount
                .as_ref()
                .map(|amount| amount.as_deref()),
            command.currency_code.as_deref(),
            command.status.as_deref(),
            command.spec,
        )
        .map_err(map_commerce_error)?;
        let record = self
            .merchandise
            .update(owner_command)
            .await
            .map_err(map_commerce_error)?;
        matter_record_from_sku(&record)
    }
}

fn matter_record_from_sku(record: &SkuRecord) -> Result<CommerceMatterRecord, NotaryServiceError> {
    Ok(CommerceMatterRecord {
        sku_id: record.id.clone(),
        spu_id: record.spu_id.clone(),
        sku_no: record.sku_no.clone(),
        title: record.title.clone(),
        description: description(record),
        price_amount: minor_units_to_major_amount(
            record.price_amount.as_str(),
            record.currency_code.as_str(),
            "priceAmount",
        )?,
        original_price_amount: record
            .original_price_amount
            .as_deref()
            .map(|amount| {
                minor_units_to_major_amount(
                    amount,
                    record.currency_code.as_str(),
                    "originalPriceAmount",
                )
            })
            .transpose()?,
        currency_code: record.currency_code.clone(),
        status: record.status.clone(),
        spec: public_spec(record),
    })
}

fn major_amount_to_minor_units(
    amount: &str,
    currency_code: &str,
    field_name: &str,
) -> Result<String, NotaryServiceError> {
    let exponent = currency_exponent(currency_code)? as usize;
    let amount = amount.trim();
    let mut parts = amount.split('.');
    let integer_part = parts.next().unwrap_or_default();
    let fraction_part = parts.next().unwrap_or_default();
    if parts.next().is_some()
        || integer_part.is_empty()
        || !integer_part.bytes().all(|byte| byte.is_ascii_digit())
        || !fraction_part.bytes().all(|byte| byte.is_ascii_digit())
        || fraction_part.len() > exponent
        || (exponent == 0 && !fraction_part.is_empty())
    {
        return Err(NotaryServiceError::validation(format!(
            "{field_name} must be a non-negative major-unit decimal with at most {exponent} fractional digits"
        )));
    }

    let integer_part = integer_part.trim_start_matches('0');
    let integer_part = if integer_part.is_empty() {
        "0"
    } else {
        integer_part
    };
    let mut fraction_part = fraction_part.to_owned();
    fraction_part.push_str(&"0".repeat(exponent - fraction_part.len()));
    let minor_units = format!("{integer_part}{fraction_part}");
    let minor_units = minor_units.trim_start_matches('0');
    Ok(if minor_units.is_empty() {
        "0".to_owned()
    } else {
        minor_units.to_owned()
    })
}

fn minor_units_to_major_amount(
    amount: &str,
    currency_code: &str,
    field_name: &str,
) -> Result<String, NotaryServiceError> {
    let exponent = currency_exponent(currency_code)? as usize;
    let amount = amount.trim();
    if amount.is_empty() || !amount.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(NotaryServiceError::storage(format!(
            "{field_name} must be stored as a non-negative smallest-unit integer"
        )));
    }

    let amount = amount.trim_start_matches('0');
    let amount = if amount.is_empty() { "0" } else { amount };
    if exponent == 0 {
        return Ok(amount.to_owned());
    }

    let padded = format!("{amount:0>width$}", width = exponent + 1);
    let split_at = padded.len() - exponent;
    Ok(format!("{}.{}", &padded[..split_at], &padded[split_at..]))
}

fn currency_exponent(currency_code: &str) -> Result<u32, NotaryServiceError> {
    let currency_code = currency_code.trim().to_ascii_uppercase();
    minor_unit_exponent(currency_code.as_str()).ok_or_else(|| {
        NotaryServiceError::validation(format!(
            "unsupported currencyCode for notary merchandise: {currency_code}"
        ))
    })
}

fn validate_comparison_price(
    price_amount: &str,
    original_price_amount: Option<&str>,
) -> Result<(), NotaryServiceError> {
    let Some(original_price_amount) = original_price_amount else {
        return Ok(());
    };
    let original_is_lower = original_price_amount.len() < price_amount.len()
        || (original_price_amount.len() == price_amount.len()
            && original_price_amount < price_amount);
    if original_is_lower {
        return Err(NotaryServiceError::validation(
            "originalPriceAmount must not be lower than priceAmount",
        ));
    }
    Ok(())
}

fn map_commerce_error(error: CommerceServiceError) -> NotaryServiceError {
    let message = error.message().to_owned();
    match error.code() {
        "unauthenticated" => NotaryServiceError::unauthenticated(message),
        "unauthorized" => NotaryServiceError::unauthorized(message),
        "not-found" => NotaryServiceError::not_found(message),
        "conflict" | "locked" => NotaryServiceError::conflict(message),
        "invalid-state" => NotaryServiceError::invalid_state(message),
        "validation" => NotaryServiceError::validation(message),
        "transport" => NotaryServiceError::transport(message),
        "unsupported-capability" | "provider-unavailable" => {
            NotaryServiceError::provider_unavailable(message)
        }
        "storage" => NotaryServiceError::storage(message),
        _ => NotaryServiceError::unknown(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sku() -> SkuRecord {
        SkuRecord {
            id: "sku-notary-1".to_owned(),
            tenant_id: "tenant-1".to_owned(),
            organization_id: Some("org-1".to_owned()),
            spu_id: "spu-notary-1".to_owned(),
            sku_no: "NOTARY-SKU-1".to_owned(),
            name: "Remote notarization".to_owned(),
            title: "Remote notarization".to_owned(),
            price_amount: "100".to_owned(),
            original_price_amount: Some("120".to_owned()),
            currency_code: "CNY".to_owned(),
            fulfillment_type: NOTARY_FULFILLMENT_TYPE.to_owned(),
            inventory_tracking: "untracked".to_owned(),
            status: "active".to_owned(),
            published_at: None,
            spec_json: Some(
                serde_json::json!({
                    "durationDays": 3,
                    "_sdkwork": {
                        "description": "Three-day service",
                        "productType": "notary",
                        "skuPolicy": "one_spu_one_sku"
                    }
                })
                .to_string(),
            ),
            created_at: "2026-07-11T00:00:00Z".to_owned(),
            updated_at: "2026-07-11T00:00:00Z".to_owned(),
        }
    }

    #[test]
    fn maps_owner_record_without_leaking_reserved_metadata() {
        let record = matter_record_from_sku(&sample_sku()).expect("mapped record");
        assert_eq!(record.description.as_deref(), Some("Three-day service"));
        assert_eq!(record.spec, serde_json::json!({"durationDays": 3}));
        assert_eq!(record.price_amount, "1.00");
        assert_eq!(record.original_price_amount.as_deref(), Some("1.20"));
    }

    #[test]
    fn converts_major_and_minor_money_without_floating_point() {
        assert_eq!(
            major_amount_to_minor_units("69.90", "CNY", "priceAmount").expect("minor units"),
            "6990"
        );
        assert_eq!(
            minor_units_to_major_amount("6990", "CNY", "priceAmount").expect("major amount"),
            "69.90"
        );
        assert_eq!(
            major_amount_to_minor_units("100", "JPY", "priceAmount").expect("JPY minor units"),
            "100"
        );
        assert!(major_amount_to_minor_units("100.00", "JPY", "priceAmount").is_err());
        assert!(major_amount_to_minor_units("1.001", "CNY", "priceAmount").is_err());
    }

    #[test]
    fn preserves_owner_error_classification() {
        assert_eq!(
            map_commerce_error(CommerceServiceError::conflict("duplicate")).code(),
            "conflict"
        );
        assert_eq!(
            map_commerce_error(CommerceServiceError::validation("invalid")).code(),
            "validation"
        );
    }
}
