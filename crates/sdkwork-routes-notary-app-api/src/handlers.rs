use std::collections::BTreeMap;

use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::Response,
    Json,
};
use serde_json::{Map, Value};

use sdkwork_routes_notary_http_auth::{
    envelope_success_data, finish_success, finish_success_no_content,
    is_delete_no_content_operation, notary_request_context_from_web,
    success_status_for_notary_app_operation, validate_list_query,
};
use sdkwork_web_core::WebRequestContext;

use crate::service_port::{NotaryAppApiState, NotaryOperationMetadata};

pub async fn retrieve_access(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
) -> Response {
    call_operation(
        state,
        app_ctx,
        "notary.access.retrieve",
        BTreeMap::new(),
        Value::Null,
    )
    .await
}

pub async fn retrieve_dashboard_statistics(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
) -> Response {
    call_operation(
        state,
        app_ctx,
        "notary.dashboard.statistics.retrieve",
        BTreeMap::new(),
        Value::Null,
    )
    .await
}

pub async fn list_matters(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
    if let Err(error) = validate_list_query(&query) {
        return error.into_response_for(&app_ctx);
    }
    call_operation(
        state,
        app_ctx,
        "notary.matters.list",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

pub async fn retrieve_monthly_report(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
    call_operation(
        state,
        app_ctx,
        "notary.reports.monthly.retrieve",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

pub async fn list_staff(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
    if let Err(error) = validate_list_query(&query) {
        return error.into_response_for(&app_ctx);
    }
    call_operation(
        state,
        app_ctx,
        "notary.staff.list",
        BTreeMap::new(),
        Value::Object(
            query
                .into_iter()
                .map(|(key, value)| (key, Value::String(value)))
                .collect(),
        ),
    )
    .await
}

pub async fn list_cases(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
    if let Err(error) = validate_list_query(&query) {
        return error.into_response_for(&app_ctx);
    }
    call_operation(
        state,
        app_ctx,
        "notary.cases.list",
        BTreeMap::new(),
        Value::Object(
            query
                .into_iter()
                .map(|(key, value)| (key, Value::String(value)))
                .collect(),
        ),
    )
    .await
}

pub async fn create_case(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    call_operation_with_metadata(
        state,
        app_ctx,
        "notary.cases.create",
        BTreeMap::new(),
        body,
        NotaryOperationMetadata {
            idempotency_key: sdkwork_web_core::extractors::idempotency_key(&headers),
        },
    )
    .await
}

pub async fn retrieve_case(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
) -> Response {
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.retrieve",
        case_id,
        Value::Null,
    )
    .await
}

pub async fn update_case(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    call_case_operation(state, app_ctx, "notary.cases.update", case_id, body).await
}

pub async fn accept_case(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    body: Option<Json<Value>>,
) -> Response {
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.acceptances.create",
        case_id,
        optional_body(body),
    )
    .await
}

pub async fn reject_case(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.rejections.create",
        case_id,
        body,
    )
    .await
}

pub async fn complete_case(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.completions.create",
        case_id,
        body,
    )
    .await
}

pub async fn create_assignment(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.assignments.create",
        case_id,
        body,
    )
    .await
}

pub async fn list_parties(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
) -> Response {
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.parties.list",
        case_id,
        Value::Null,
    )
    .await
}

pub async fn create_party(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    call_case_operation(state, app_ctx, "notary.cases.parties.create", case_id, body).await
}

pub async fn update_party(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path((case_id, party_id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Response {
    call_party_operation(
        state,
        app_ctx,
        "notary.cases.parties.update",
        case_id,
        party_id,
        body,
    )
    .await
}

pub async fn delete_party(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path((case_id, party_id)): Path<(String, String)>,
) -> Response {
    call_party_operation(
        state,
        app_ctx,
        "notary.cases.parties.delete",
        case_id,
        party_id,
        Value::Null,
    )
    .await
}

pub async fn attach_party_signature(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path((case_id, party_id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Response {
    call_party_operation(
        state,
        app_ctx,
        "notary.cases.parties.signatures.create",
        case_id,
        party_id,
        body,
    )
    .await
}

pub async fn create_party_video_invite(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path((case_id, party_id)): Path<(String, String)>,
    body: Option<Json<Value>>,
) -> Response {
    call_party_operation(
        state,
        app_ctx,
        "notary.cases.parties.videoInvites.create",
        case_id,
        party_id,
        optional_body(body),
    )
    .await
}

pub async fn create_party_signature_invite(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path((case_id, party_id)): Path<(String, String)>,
    body: Option<Json<Value>>,
) -> Response {
    call_party_operation(
        state,
        app_ctx,
        "notary.cases.parties.signatureInvites.create",
        case_id,
        party_id,
        optional_body(body),
    )
    .await
}

pub async fn list_case_files(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
    if let Err(error) = validate_list_query(&query) {
        return error.into_response_for(&app_ctx);
    }
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.files.list",
        case_id,
        query_body(query),
    )
    .await
}

pub async fn create_case_file(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Response {
    call_case_operation(state, app_ctx, "notary.cases.files.create", case_id, body).await
}

pub async fn create_download_package(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    body: Option<Json<Value>>,
) -> Response {
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.downloadPackages.create",
        case_id,
        optional_body(body),
    )
    .await
}

pub async fn list_case_events(
    State(state): State<NotaryAppApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
    if let Err(error) = validate_list_query(&query) {
        return error.into_response_for(&app_ctx);
    }
    call_case_operation(
        state,
        app_ctx,
        "notary.cases.events.list",
        case_id,
        query_body(query),
    )
    .await
}

async fn call_case_operation(
    state: NotaryAppApiState,
    app_ctx: WebRequestContext,
    operation_id: &'static str,
    case_id: String,
    body: Value,
) -> Response {
    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), case_id);
    call_operation(state, app_ctx, operation_id, path_params, body).await
}

async fn call_party_operation(
    state: NotaryAppApiState,
    app_ctx: WebRequestContext,
    operation_id: &'static str,
    case_id: String,
    party_id: String,
    body: Value,
) -> Response {
    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), case_id);
    path_params.insert("partyId".to_string(), party_id);
    call_operation(state, app_ctx, operation_id, path_params, body).await
}

async fn call_operation(
    state: NotaryAppApiState,
    app_ctx: WebRequestContext,
    operation_id: &'static str,
    path_params: BTreeMap<String, String>,
    body: Value,
) -> Response {
    call_operation_with_metadata(
        state,
        app_ctx,
        operation_id,
        path_params,
        body,
        NotaryOperationMetadata::default(),
    )
    .await
}

async fn call_operation_with_metadata(
    state: NotaryAppApiState,
    app_ctx: WebRequestContext,
    operation_id: &'static str,
    path_params: BTreeMap<String, String>,
    body: Value,
    metadata: NotaryOperationMetadata,
) -> Response {
    let result = async {
        let request_context = notary_request_context_from_web(&app_ctx)?;
        let service = state.service().clone();
        let response = service
            .handle(request_context, operation_id, path_params, body, metadata)
            .await?;
        if is_delete_no_content_operation(operation_id) {
            return finish_success_no_content(&app_ctx);
        }
        finish_success(
            &app_ctx,
            success_status_for_notary_app_operation(operation_id),
            envelope_success_data(operation_id, response),
        )
    }
    .await;

    match result {
        Ok(response) => response,
        Err(error) => error.into_response_for(&app_ctx),
    }
}

fn optional_body(body: Option<Json<Value>>) -> Value {
    body.map(|Json(value)| value)
        .unwrap_or_else(|| Value::Object(Map::new()))
}

fn query_body(query: BTreeMap<String, String>) -> Value {
    Value::Object(
        query
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect(),
    )
}
