use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use async_trait::async_trait;
use axum::{
    body::to_bytes,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
    Json,
};
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_notary_case_repository_sqlx::{
    notary_foundation_migration_sql, SqliteNotaryCaseRepository,
};
use sdkwork_notary_case_service::{
    AppbaseOrganizationMember, AppbasePort, CommerceCreateOrderCommand, CommerceOrderReference,
    CommercePort, DriveCreateFolderCommand, DriveCreateSpaceCommand, DriveFolderReference,
    DriveListNodesPage, DriveListNodesQuery, DrivePort,
};
use sdkwork_routes_notary_app_api::{
    handlers, service_port::NotaryAppApiState, NotaryAppApiServicePort, NotaryAppRuntimeService,
};
use sdkwork_web_core::{
    ServerRequestId, WebApiSurface, WebAuthMode, WebRequestContext, WebRequestPrincipal,
    WebTransportFacts,
};
use serde_json::{json, Value};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

const TENANT_ID: &str = "100001";
const ORGANIZATION_ID: &str = "200001";
const USER_ID: &str = "1";
const MEMBERSHIP_ID: &str = "member-notary-1";
const TEST_PII_VAULT_KEY: &str = "sdkwork-notary-test-pii-vault-key";

#[tokio::test]
async fn sqlite_app_api_flow_preserves_header_idempotency_and_persistent_state() {
    std::env::set_var("NOTARY_PII_VAULT_KEY", TEST_PII_VAULT_KEY);
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), TENANT_ID, USER_ID);
    repository
        .upsert_organization_profile(ORGANIZATION_ID, "drive-space-notary-1", "notary")
        .await
        .expect("notary organization profile");

    let commerce_create_calls = Arc::new(AtomicUsize::new(0));
    let drive_folder_create_calls = Arc::new(AtomicUsize::new(0));
    let service: Arc<dyn NotaryAppApiServicePort> = Arc::new(NotaryAppRuntimeService::new(
        TestOnlyIamPort,
        TestOnlyCommercePort {
            create_calls: Arc::clone(&commerce_create_calls),
        },
        TestOnlyDrivePort {
            folder_create_calls: Arc::clone(&drive_folder_create_calls),
        },
        repository,
    ));
    let state = NotaryAppApiState::new(service);
    let context = test_web_request_context();

    let (first_status, first_payload) = create_case(
        state.clone(),
        context.clone(),
        "notary-create-flow-1",
        "Electronic contract preservation",
    )
    .await;
    assert_eq!(first_status, StatusCode::CREATED);
    let first_item = resource_item(&first_payload);
    let first_case_id = required_string(first_item, "caseId").to_owned();
    assert_eq!(first_item["status"], "PENDING_REVIEW");
    assert_eq!(first_item["version"], "1");

    let (duplicate_status, duplicate_payload) = create_case(
        state.clone(),
        context.clone(),
        "notary-create-flow-1",
        "Electronic contract preservation",
    )
    .await;
    assert_eq!(duplicate_status, StatusCode::CREATED);
    let duplicate_item = resource_item(&duplicate_payload);
    assert_eq!(required_string(duplicate_item, "caseId"), first_case_id);
    assert_eq!(duplicate_item["title"], "Electronic contract preservation");

    let persisted_after_duplicate: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM notary_case WHERE tenant_id = ?1 AND organization_id = ?2",
    )
    .bind(TENANT_ID)
    .bind(ORGANIZATION_ID)
    .fetch_one(&pool)
    .await
    .expect("count persisted cases after duplicate create");
    assert_eq!(persisted_after_duplicate, 1);
    assert_eq!(commerce_create_calls.load(Ordering::SeqCst), 1);
    assert_eq!(drive_folder_create_calls.load(Ordering::SeqCst), 1);

    for (idempotency_key, title) in [
        ("notary-create-flow-2", "Power of attorney preservation"),
        ("notary-create-flow-3", "Evidence preservation"),
    ] {
        let (status, _) = create_case(state.clone(), context.clone(), idempotency_key, title).await;
        assert_eq!(status, StatusCode::CREATED);
    }

    let (first_page_status, first_page) = list_cases(state.clone(), context.clone(), None, 2).await;
    assert_eq!(first_page_status, StatusCode::OK);
    let first_page_items = first_page["data"]["items"]
        .as_array()
        .expect("first cursor page items");
    assert_eq!(first_page_items.len(), 2);
    assert_eq!(first_page["data"]["pageInfo"]["mode"], "cursor");
    assert_eq!(first_page["data"]["pageInfo"]["pageSize"], 2);
    assert_eq!(first_page["data"]["pageInfo"]["totalItems"], "3");
    assert_eq!(first_page["data"]["pageInfo"]["hasMore"], true);
    let next_cursor = required_string(&first_page["data"]["pageInfo"], "nextCursor");

    let (second_page_status, second_page) =
        list_cases(state.clone(), context.clone(), Some(next_cursor), 2).await;
    assert_eq!(second_page_status, StatusCode::OK);
    let second_page_items = second_page["data"]["items"]
        .as_array()
        .expect("second cursor page items");
    assert_eq!(second_page_items.len(), 1);
    assert_eq!(second_page["data"]["pageInfo"]["totalItems"], "3");
    assert_eq!(second_page["data"]["pageInfo"]["hasMore"], false);

    let mut listed_case_ids = first_page_items
        .iter()
        .chain(second_page_items.iter())
        .map(|item| required_string(item, "caseId").to_owned())
        .collect::<Vec<_>>();
    listed_case_ids.sort();
    listed_case_ids.dedup();
    assert_eq!(listed_case_ids.len(), 3);
    assert!(listed_case_ids.contains(&first_case_id));

    let retrieve_response = handlers::retrieve_case(
        State(state.clone()),
        context.clone(),
        Path(first_case_id.clone()),
    )
    .await;
    let (retrieve_status, retrieve_payload) = response_json(retrieve_response).await;
    assert_eq!(retrieve_status, StatusCode::OK);
    assert_eq!(
        required_string(resource_item(&retrieve_payload), "caseId"),
        first_case_id
    );
    assert_eq!(resource_item(&retrieve_payload)["status"], "PENDING_REVIEW");
    assert!(resource_item(&retrieve_payload)["timeline"]
        .as_array()
        .expect("persisted case timeline")
        .iter()
        .any(|event| event["eventType"] == "notary.case.submitted"));

    let accept_response = handlers::accept_case(
        State(state.clone()),
        context.clone(),
        Path(first_case_id.clone()),
        None,
    )
    .await;
    let (accept_status, accept_payload) = response_json(accept_response).await;
    assert_eq!(accept_status, StatusCode::CREATED);
    assert_eq!(resource_item(&accept_payload)["status"], "PROCESSING");
    assert_eq!(resource_item(&accept_payload)["version"], "2");

    let stale_update_response = handlers::update_case(
        State(state.clone()),
        context.clone(),
        Path(first_case_id.clone()),
        Json(json!({
            "title": "stale update must not win",
            "version": "1"
        })),
    )
    .await;
    let (stale_status, stale_payload) = response_json(stale_update_response).await;
    assert_eq!(stale_status, StatusCode::CONFLICT);
    assert_eq!(stale_payload["code"], 40901);
    assert_eq!(stale_payload["traceId"], "trace-notary-sqlite-flow");
    assert_eq!(stale_payload["detail"], "notary case version conflict");

    let complete_response = handlers::complete_case(
        State(state),
        context,
        Path(first_case_id.clone()),
        Json(json!({ "chainHash": "sha256:notary-create-flow-1" })),
    )
    .await;
    let (complete_status, complete_payload) = response_json(complete_response).await;
    assert_eq!(complete_status, StatusCode::CREATED);
    assert_eq!(resource_item(&complete_payload)["status"], "COMPLETED");
    assert_eq!(resource_item(&complete_payload)["version"], "3");
    assert_eq!(
        resource_item(&complete_payload)["chainHash"],
        "sha256:notary-create-flow-1"
    );

    let persisted_state: (String, Option<String>, i64, String) = sqlx::query_as(
        "SELECT status, chain_hash, version, title FROM notary_case WHERE tenant_id = ?1 AND id = ?2",
    )
    .bind(TENANT_ID)
    .bind(&first_case_id)
    .fetch_one(&pool)
    .await
    .expect("completed notary case row");
    assert_eq!(persisted_state.0, "completed");
    assert_eq!(
        persisted_state.1.as_deref(),
        Some("sha256:notary-create-flow-1")
    );
    assert_eq!(persisted_state.2, 3);
    assert_eq!(persisted_state.3, "Electronic contract preservation");
}

async fn create_case(
    state: NotaryAppApiState,
    context: WebRequestContext,
    idempotency_key: &'static str,
    title: &str,
) -> (StatusCode, Value) {
    let mut headers = HeaderMap::new();
    headers.insert("Idempotency-Key", HeaderValue::from_static(idempotency_key));
    let response = handlers::create_case(
        State(state),
        context,
        headers,
        Json(json!({
            "organizationId": ORGANIZATION_ID,
            "skuId": "sku-notary-contract",
            "title": title,
            "applicantName": "SDKWork Integration Applicant",
            "idempotencyKey": "body-value-must-be-ignored"
        })),
    )
    .await;
    response_json(response).await
}

async fn list_cases(
    state: NotaryAppApiState,
    context: WebRequestContext,
    cursor: Option<&str>,
    page_size: i64,
) -> (StatusCode, Value) {
    let mut query =
        std::collections::BTreeMap::from([("page_size".to_owned(), page_size.to_string())]);
    if let Some(cursor) = cursor {
        query.insert("cursor".to_owned(), cursor.to_owned());
    }
    let response = handlers::list_cases(State(state), context, Query(query)).await;
    response_json(response).await
}

async fn response_json(response: Response) -> (StatusCode, Value) {
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body");
    let payload = serde_json::from_slice(&body).expect("JSON response body");
    (status, payload)
}

fn resource_item(payload: &Value) -> &Value {
    payload["data"]["item"]
        .as_object()
        .map(|_| &payload["data"]["item"])
        .expect("resource response item")
}

fn required_string<'a>(value: &'a Value, field: &str) -> &'a str {
    value[field].as_str().unwrap_or_else(|| panic!("{field}"))
}

async fn migrated_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("in-memory SQLite pool");
    sqlx::raw_sql(notary_foundation_migration_sql())
        .execute(&pool)
        .await
        .expect("notary SQLite migration");
    pool
}

fn test_web_request_context() -> WebRequestContext {
    WebRequestContext {
        request_id: ServerRequestId("req-notary-sqlite-flow".to_owned()),
        api_surface: WebApiSurface::AppApi,
        auth_mode: WebAuthMode::DualToken,
        principal: Some(
            WebRequestPrincipal::builder()
                .tenant_id(TENANT_ID)
                .organization_id(Some(ORGANIZATION_ID.to_owned()))
                .user_id(USER_ID)
                .session_id(Some("session-notary-sqlite-flow".to_owned()))
                .app_id("sdkwork-notary-pc")
                .data_scope(vec![format!("organization_membership:{MEMBERSHIP_ID}")])
                .permission_scope(vec!["notary.*".to_owned()])
                .build(),
        ),
        transport: WebTransportFacts {
            path: "/app/v3/api/notary/cases".to_owned(),
            method: "POST".to_owned(),
            auth_token_present: true,
            access_token_present: true,
            api_key_present: false,
            oauth_bearer_present: false,
            agent_token_present: false,
        },
        locale: None,
        client_kind: None,
        operation: None,
        trace_id: Some("trace-notary-sqlite-flow".to_owned()),
    }
}

#[derive(Clone, Copy)]
struct TestOnlyIamPort;

#[async_trait]
impl AppbasePort for TestOnlyIamPort {
    async fn get_organization_member(
        &self,
        organization_id: &str,
        membership_id: &str,
    ) -> Result<Option<AppbaseOrganizationMember>, NotaryServiceError> {
        Ok(
            (organization_id == ORGANIZATION_ID && membership_id == MEMBERSHIP_ID)
                .then(test_notary_member),
        )
    }

    async fn list_organization_members(
        &self,
        organization_id: &str,
    ) -> Result<Vec<AppbaseOrganizationMember>, NotaryServiceError> {
        Ok(if organization_id == ORGANIZATION_ID {
            vec![test_notary_member()]
        } else {
            Vec::new()
        })
    }
}

fn test_notary_member() -> AppbaseOrganizationMember {
    AppbaseOrganizationMember {
        membership_id: MEMBERSHIP_ID.to_owned(),
        user_id: USER_ID.to_owned(),
        organization_id: ORGANIZATION_ID.to_owned(),
        display_name: "SDKWork Test Notary".to_owned(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary_admin".to_owned()],
        positions: vec!["notary".to_owned()],
        departments: vec!["notary".to_owned()],
    }
}

#[derive(Clone)]
struct TestOnlyCommercePort {
    create_calls: Arc<AtomicUsize>,
}

#[async_trait]
impl CommercePort for TestOnlyCommercePort {
    async fn create_notary_order(
        &self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError> {
        self.create_calls.fetch_add(1, Ordering::SeqCst);
        Ok(CommerceOrderReference {
            order_id: format!("order-{}", command.idempotency_key),
            order_item_id: format!("item-{}", command.idempotency_key),
            sku_id: command.sku_id,
            matter_title: command.title,
            fee_amount: "500.00".to_owned(),
            currency_code: "CNY".to_owned(),
        })
    }

    async fn cancel_notary_order(&self, _order_id: &str) -> Result<(), NotaryServiceError> {
        Ok(())
    }
}

#[derive(Clone)]
struct TestOnlyDrivePort {
    folder_create_calls: Arc<AtomicUsize>,
}

#[async_trait]
impl DrivePort for TestOnlyDrivePort {
    async fn create_notary_space(
        &self,
        _command: DriveCreateSpaceCommand,
    ) -> Result<String, NotaryServiceError> {
        Ok("drive-space-notary-1".to_owned())
    }

    async fn create_case_folder(
        &self,
        command: DriveCreateFolderCommand,
    ) -> Result<DriveFolderReference, NotaryServiceError> {
        self.folder_create_calls.fetch_add(1, Ordering::SeqCst);
        Ok(DriveFolderReference {
            folder_node_id: format!("folder-{}", command.case_id),
            space_id: command.space_id,
            space_type: command.space_type,
        })
    }

    async fn delete_case_folder(
        &self,
        _folder_node_id: &str,
        _space_id: &str,
        _space_type: &str,
    ) -> Result<(), NotaryServiceError> {
        Ok(())
    }

    async fn list_nodes(
        &self,
        _query: DriveListNodesQuery,
    ) -> Result<DriveListNodesPage, NotaryServiceError> {
        Ok(DriveListNodesPage {
            items: Vec::new(),
            has_more: false,
            next_cursor: None,
        })
    }
}
