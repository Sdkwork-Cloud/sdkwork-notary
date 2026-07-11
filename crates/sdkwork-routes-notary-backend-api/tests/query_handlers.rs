use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::{
    body::to_bytes,
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use sdkwork_routes_notary_backend_api::{
    handlers,
    service_port::{NotaryBackendApiState, NotaryRouteError},
    NotaryBackendApiServicePort, NotaryRequestContext,
};
use sdkwork_routes_notary_http_auth::test_support::test_backend_web_request_context;
use serde_json::Value;

#[tokio::test]
async fn backend_list_handlers_forward_query_filters_to_service_body() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryBackendApiState::new(service.clone());
    let app_ctx = test_backend_web_request_context();

    let _ = handlers::list_cases(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([
            ("organization_id".to_string(), "200001".to_string()),
            ("status".to_string(), "PROCESSING".to_string()),
            ("q".to_string(), "contract".to_string()),
            ("page_size".to_string(), "100".to_string()),
        ])),
    )
    .await;

    let _ = handlers::list_staff(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([
            ("organization_id".to_string(), "200001".to_string()),
            ("staff_role".to_string(), "notary".to_string()),
        ])),
    )
    .await;

    let _ = handlers::retrieve_case_summary(
        State(state),
        app_ctx,
        Query(BTreeMap::from([
            ("organization_id".to_string(), "200001".to_string()),
            (
                "created_after".to_string(),
                "2026-06-01T00:00:00Z".to_string(),
            ),
            (
                "created_before".to_string(),
                "2026-06-10T23:59:59Z".to_string(),
            ),
        ])),
    )
    .await;

    let calls = service.calls.lock().unwrap();
    assert_eq!(calls[0].operation_id, "notary.cases.management.list");
    assert_eq!(calls[0].body["organization_id"], "200001");
    assert_eq!(calls[0].body["status"], "PROCESSING");
    assert_eq!(calls[0].body["q"], "contract");
    assert_eq!(calls[0].body["page_size"], "100");

    assert_eq!(calls[1].operation_id, "notary.staff.list");
    assert_eq!(calls[1].body["organization_id"], "200001");
    assert_eq!(calls[1].body["staff_role"], "notary");

    assert_eq!(calls[2].operation_id, "notary.reports.caseSummary.retrieve");
    assert_eq!(calls[2].body["organization_id"], "200001");
    assert_eq!(calls[2].body["created_after"], "2026-06-01T00:00:00Z");
    assert_eq!(calls[2].body["created_before"], "2026-06-10T23:59:59Z");
}

#[tokio::test]
async fn backend_profile_and_matter_list_handlers_forward_query_filters_to_service_body() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryBackendApiState::new(service.clone());
    let app_ctx = test_backend_web_request_context();

    let _ = handlers::list_organization_profiles(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([
            ("organization_id".to_string(), "200001".to_string()),
            ("page_size".to_string(), "10".to_string()),
        ])),
    )
    .await;

    let _ = handlers::list_matters(
        State(state),
        app_ctx,
        Query(BTreeMap::from([
            ("organization_id".to_string(), "200001".to_string()),
            ("q".to_string(), "contract".to_string()),
            ("page_size".to_string(), "20".to_string()),
        ])),
    )
    .await;

    let calls = service.calls.lock().unwrap();
    assert_eq!(calls[0].operation_id, "notary.organizationProfiles.list");
    assert_eq!(calls[0].body["organization_id"], "200001");
    assert_eq!(calls[0].body["page_size"], "10");

    assert_eq!(calls[1].operation_id, "notary.matters.management.list");
    assert_eq!(calls[1].body["organization_id"], "200001");
    assert_eq!(calls[1].body["q"], "contract");
    assert_eq!(calls[1].body["page_size"], "20");
}

#[tokio::test]
async fn backend_list_query_rejections_return_invalid_parameter_before_service_dispatch() {
    let service = Arc::new(RecordingService::default());
    let state = NotaryBackendApiState::new(service.clone());
    let app_ctx = test_backend_web_request_context();

    let profiles_response = handlers::list_organization_profiles(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([("limit".to_string(), "20".to_string())])),
    )
    .await;
    assert_invalid_parameter(profiles_response).await;

    let matters_response = handlers::list_matters(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([
            ("page".to_string(), "1".to_string()),
            ("cursor".to_string(), "next-page".to_string()),
        ])),
    )
    .await;
    assert_invalid_parameter(matters_response).await;

    let cases_response = handlers::list_cases(
        State(state.clone()),
        app_ctx.clone(),
        Query(BTreeMap::from([(
            "page_size".to_string(),
            "201".to_string(),
        )])),
    )
    .await;
    assert_invalid_parameter(cases_response).await;

    let staff_response = handlers::list_staff(
        State(state),
        app_ctx,
        Query(BTreeMap::from([(
            "page_size".to_string(),
            "staff".to_string(),
        )])),
    )
    .await;
    assert_invalid_parameter(staff_response).await;

    assert!(
        service.calls.lock().unwrap().is_empty(),
        "invalid list queries must not reach the service"
    );
}

#[derive(Default)]
struct RecordingService {
    calls: Mutex<Vec<RecordedCall>>,
}

struct RecordedCall {
    operation_id: &'static str,
    body: Value,
}

#[async_trait]
impl NotaryBackendApiServicePort for RecordingService {
    async fn handle(
        &self,
        _context: NotaryRequestContext,
        operation_id: &'static str,
        _path_params: BTreeMap<String, String>,
        body: Value,
    ) -> Result<Value, NotaryRouteError> {
        self.calls
            .lock()
            .unwrap()
            .push(RecordedCall { operation_id, body });
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
