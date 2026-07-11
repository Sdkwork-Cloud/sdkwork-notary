use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::{
    body::to_bytes,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
    Json,
};
use sdkwork_routes_notary_app_api::{
    handlers, notary_app_api_http_route_manifest,
    service_port::{NotaryAppApiState, NotaryRouteError},
    NotaryAppApiServicePort, NotaryOperationMetadata, NotaryRequestContext,
};
use sdkwork_routes_notary_http_auth::test_support::test_web_request_context;
use serde_json::Value;

#[tokio::test]
async fn list_cases_handler_forwards_query_filters_to_service_body() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryAppApiState::new(service.clone());

    let _ = handlers::list_cases(
        State(state),
        test_web_request_context(),
        Query(BTreeMap::from([
            ("status".to_string(), "PROCESSING".to_string()),
            ("q".to_string(), "contract".to_string()),
            ("page_size".to_string(), "100".to_string()),
        ])),
    )
    .await;

    let calls = service.calls.lock().unwrap();
    assert_eq!(calls[0].operation_id, "notary.cases.list");
    assert_eq!(calls[0].body["status"], "PROCESSING");
    assert_eq!(calls[0].body["q"], "contract");
    assert_eq!(calls[0].body["page_size"], "100");
}

#[tokio::test]
async fn app_list_query_rejections_return_invalid_parameter_before_service_dispatch() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryAppApiState::new(service.clone());
    let app_ctx = test_web_request_context();

    let alias_response = handlers::list_cases(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([("pageSize".to_string(), "20".to_string())])),
    )
    .await;
    assert_invalid_parameter(alias_response).await;

    let combination_response = handlers::list_matters(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([
            ("page".to_string(), "1".to_string()),
            ("cursor".to_string(), "next-page".to_string()),
        ])),
    )
    .await;
    assert_invalid_parameter(combination_response).await;

    let staff_response = handlers::list_staff(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([(
            "page_size".to_string(),
            "201".to_string(),
        )])),
    )
    .await;
    assert_invalid_parameter(staff_response).await;

    let files_response = handlers::list_case_files(
        State(state.clone()),
        app_ctx.clone(),
        Path("case-1".to_string()),
        Query(BTreeMap::from([("page_size".to_string(), "0".to_string())])),
    )
    .await;
    assert_invalid_parameter(files_response).await;

    let events_response = handlers::list_case_events(
        State(state),
        app_ctx,
        Path("case-1".to_string()),
        Query(BTreeMap::from([(
            "page_size".to_string(),
            "many".to_string(),
        )])),
    )
    .await;
    assert_invalid_parameter(events_response).await;

    assert!(
        service.calls.lock().unwrap().is_empty(),
        "invalid list queries must not reach the service"
    );
}

#[tokio::test]
async fn create_case_handler_forwards_standard_idempotency_header_as_metadata() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryAppApiState::new(service.clone());
    let mut headers = HeaderMap::new();
    headers.insert(
        "Idempotency-Key",
        HeaderValue::from_static("case-create-request-1"),
    );
    let body = serde_json::json!({
        "organizationId": "200001",
        "skuId": "sku-electronic-contract",
        "title": "Electronic contract preservation",
        "applicantName": "Zhang San Network"
    });

    let _ = handlers::create_case(
        State(state),
        test_web_request_context(),
        headers,
        Json(body),
    )
    .await;

    let calls = service.calls.lock().unwrap();
    assert_eq!(calls[0].operation_id, "notary.cases.create");
    assert_eq!(
        calls[0].metadata.idempotency_key.as_deref(),
        Some("case-create-request-1")
    );
    assert!(calls[0].body.get("idempotencyKey").is_none());
}

#[test]
fn create_case_route_policy_is_idempotent() {
    let route = notary_app_api_http_route_manifest()
        .routes()
        .iter()
        .find(|route| route.operation_id == "notary.cases.create")
        .expect("notary.cases.create route");

    assert!(route.idempotent);
}

#[tokio::test]
async fn case_action_create_handlers_return_created_status() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryAppApiState::new(service);
    let app_ctx = test_web_request_context();

    let accepted = handlers::accept_case(
        State(state.clone()),
        app_ctx.clone(),
        Path("case-1".to_string()),
        None,
    )
    .await;
    let rejected = handlers::reject_case(
        State(state.clone()),
        app_ctx.clone(),
        Path("case-1".to_string()),
        Json(serde_json::json!({"reason": "invalid evidence"})),
    )
    .await;
    let completed = handlers::complete_case(
        State(state),
        app_ctx,
        Path("case-1".to_string()),
        Json(serde_json::json!({"chainHash": "sha256:case-1"})),
    )
    .await;

    assert_eq!(accepted.status(), StatusCode::CREATED);
    assert_eq!(rejected.status(), StatusCode::CREATED);
    assert_eq!(completed.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn app_list_handlers_forward_all_openapi_query_filters_to_service_body() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryAppApiState::new(service.clone());
    let app_ctx = test_web_request_context();

    let _ = handlers::list_matters(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([
            ("q".to_string(), "contract".to_string()),
            ("page_size".to_string(), "20".to_string()),
            ("cursor".to_string(), "matter-cursor".to_string()),
        ])),
    )
    .await;

    let _ = handlers::list_staff(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([
            ("q".to_string(), "Li".to_string()),
            ("staff_role".to_string(), "notary".to_string()),
            ("page_size".to_string(), "10".to_string()),
            ("cursor".to_string(), "staff-cursor".to_string()),
        ])),
    )
    .await;

    let _ = handlers::list_case_files(
        State(state.clone()),
        app_ctx.clone(),
        Path("case-1".to_string()),
        Query(BTreeMap::from([
            ("category".to_string(), "identity".to_string()),
            ("page_size".to_string(), "25".to_string()),
            ("cursor".to_string(), "file-cursor".to_string()),
        ])),
    )
    .await;

    let _ = handlers::list_case_events(
        State(state),
        app_ctx,
        Path("case-1".to_string()),
        Query(BTreeMap::from([
            ("page_size".to_string(), "50".to_string()),
            ("cursor".to_string(), "event-cursor".to_string()),
        ])),
    )
    .await;

    let calls = service.calls.lock().unwrap();
    assert_eq!(calls[0].operation_id, "notary.matters.list");
    assert_eq!(calls[0].body["q"], "contract");
    assert_eq!(calls[0].body["page_size"], "20");
    assert_eq!(calls[0].body["cursor"], "matter-cursor");

    assert_eq!(calls[1].operation_id, "notary.staff.list");
    assert_eq!(calls[1].body["q"], "Li");
    assert_eq!(calls[1].body["staff_role"], "notary");
    assert_eq!(calls[1].body["page_size"], "10");
    assert_eq!(calls[1].body["cursor"], "staff-cursor");

    assert_eq!(calls[2].operation_id, "notary.cases.files.list");
    assert_eq!(calls[2].body["category"], "identity");
    assert_eq!(calls[2].body["page_size"], "25");
    assert_eq!(calls[2].body["cursor"], "file-cursor");

    assert_eq!(calls[3].operation_id, "notary.cases.events.list");
    assert_eq!(calls[3].body["page_size"], "50");
    assert_eq!(calls[3].body["cursor"], "event-cursor");
}

#[tokio::test]
async fn dashboard_and_report_handlers_forward_app_operations_to_service() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryAppApiState::new(service.clone());
    let app_ctx = test_web_request_context();

    let _ = handlers::retrieve_dashboard_statistics(State(state.clone()), app_ctx.clone()).await;

    let _ = handlers::retrieve_monthly_report(
        State(state),
        app_ctx,
        Query(BTreeMap::from([
            ("month".to_string(), "2026-06".to_string()),
            ("format".to_string(), "csv".to_string()),
        ])),
    )
    .await;

    let calls = service.calls.lock().unwrap();
    assert_eq!(
        calls[0].operation_id,
        "notary.dashboard.statistics.retrieve"
    );
    assert_eq!(calls[0].body, Value::Null);

    assert_eq!(calls[1].operation_id, "notary.reports.monthly.retrieve");
    assert_eq!(calls[1].body["month"], "2026-06");
    assert_eq!(calls[1].body["format"], "csv");
}

#[derive(Default)]
struct RecordingService {
    calls: Mutex<Vec<RecordedCall>>,
}

struct RecordedCall {
    operation_id: &'static str,
    body: Value,
    metadata: NotaryOperationMetadata,
}

#[async_trait]
impl NotaryAppApiServicePort for RecordingService {
    async fn handle(
        &self,
        _context: NotaryRequestContext,
        operation_id: &'static str,
        _path_params: BTreeMap<String, String>,
        body: Value,
        metadata: NotaryOperationMetadata,
    ) -> Result<Value, NotaryRouteError> {
        self.calls.lock().unwrap().push(RecordedCall {
            operation_id,
            body,
            metadata,
        });
        Ok(serde_json::json!({
            "items": [],
            "pageInfo": {
                "hasMore": false
            }
        }))
    }
}

async fn assert_invalid_parameter(response: Response) {
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("application/problem+json")
    );
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("problem body");
    let payload: Value = serde_json::from_slice(&body).expect("problem json");
    assert_eq!(payload["code"].as_i64(), Some(40003));
}
