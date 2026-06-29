use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::extract::{Path, Query, State};
use sdkwork_routes_notary_app_api::{
    handlers,
    service_port::{NotaryAppApiState, NotaryRouteError},
    NotaryAppApiServicePort, NotaryRequestContext,
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
            ("pageSize".to_string(), "25".to_string()),
        ])),
    )
    .await;

    let calls = service.calls.lock().unwrap();
    assert_eq!(calls[0].operation_id, "notary.cases.list");
    assert_eq!(calls[0].body["status"], "PROCESSING");
    assert_eq!(calls[0].body["q"], "contract");
    assert_eq!(calls[0].body["pageSize"], "25");
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
}

#[async_trait]
impl NotaryAppApiServicePort for RecordingService {
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
