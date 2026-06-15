use std::collections::BTreeMap;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::{Map, Value};

use crate::service_port::{NotaryAppApiState, NotaryRouteError};

pub async fn retrieve_access(
    State(state): State<NotaryAppApiState>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        "notary.access.retrieve",
        BTreeMap::new(),
        Value::Null,
    )
    .await
}

pub async fn retrieve_dashboard_statistics(
    State(state): State<NotaryAppApiState>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        "notary.dashboard.statistics.retrieve",
        BTreeMap::new(),
        Value::Null,
    )
    .await
}

pub async fn list_matters(
    State(state): State<NotaryAppApiState>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        "notary.matters.list",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

pub async fn retrieve_monthly_report(
    State(state): State<NotaryAppApiState>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        "notary.reports.monthly.retrieve",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

pub async fn list_staff(
    State(state): State<NotaryAppApiState>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
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
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
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
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(state, "notary.cases.create", BTreeMap::new(), body).await
}

pub async fn retrieve_case(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.retrieve", case_id, Value::Null).await
}

pub async fn update_case(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.update", case_id, body).await
}

pub async fn accept_case(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    body: Option<Json<Value>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(
        state,
        "notary.cases.acceptances.create",
        case_id,
        optional_body(body),
    )
    .await
}

pub async fn reject_case(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.rejections.create", case_id, body).await
}

pub async fn complete_case(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.completions.create", case_id, body).await
}

pub async fn create_assignment(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.assignments.create", case_id, body).await
}

pub async fn list_parties(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.parties.list", case_id, Value::Null).await
}

pub async fn create_party(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.parties.create", case_id, body).await
}

pub async fn update_party(
    State(state): State<NotaryAppApiState>,
    Path((case_id, party_id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_party_operation(
        state,
        "notary.cases.parties.update",
        case_id,
        party_id,
        body,
    )
    .await
}

pub async fn delete_party(
    State(state): State<NotaryAppApiState>,
    Path((case_id, party_id)): Path<(String, String)>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_party_operation(
        state,
        "notary.cases.parties.delete",
        case_id,
        party_id,
        Value::Null,
    )
    .await
}

pub async fn attach_party_signature(
    State(state): State<NotaryAppApiState>,
    Path((case_id, party_id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_party_operation(
        state,
        "notary.cases.parties.signatures.create",
        case_id,
        party_id,
        body,
    )
    .await
}

pub async fn create_party_video_invite(
    State(state): State<NotaryAppApiState>,
    Path((case_id, party_id)): Path<(String, String)>,
    body: Option<Json<Value>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_party_operation(
        state,
        "notary.cases.parties.videoInvites.create",
        case_id,
        party_id,
        optional_body(body),
    )
    .await
}

pub async fn create_party_signature_invite(
    State(state): State<NotaryAppApiState>,
    Path((case_id, party_id)): Path<(String, String)>,
    body: Option<Json<Value>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_party_operation(
        state,
        "notary.cases.parties.signatureInvites.create",
        case_id,
        party_id,
        optional_body(body),
    )
    .await
}

pub async fn list_case_files(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.files.list", case_id, query_body(query)).await
}

pub async fn create_case_file(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(state, "notary.cases.files.create", case_id, body).await
}

pub async fn create_download_package(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    body: Option<Json<Value>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(
        state,
        "notary.cases.downloadPackages.create",
        case_id,
        optional_body(body),
    )
    .await
}

pub async fn list_case_events(
    State(state): State<NotaryAppApiState>,
    Path(case_id): Path<String>,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_case_operation(
        state,
        "notary.cases.events.list",
        case_id,
        query_body(query),
    )
    .await
}

async fn call_case_operation(
    state: NotaryAppApiState,
    operation_id: &'static str,
    case_id: String,
    body: Value,
) -> Result<Json<Value>, NotaryRouteError> {
    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), case_id);
    call_operation(state, operation_id, path_params, body).await
}

async fn call_party_operation(
    state: NotaryAppApiState,
    operation_id: &'static str,
    case_id: String,
    party_id: String,
    body: Value,
) -> Result<Json<Value>, NotaryRouteError> {
    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), case_id);
    path_params.insert("partyId".to_string(), party_id);
    call_operation(state, operation_id, path_params, body).await
}

async fn call_operation(
    state: NotaryAppApiState,
    operation_id: &'static str,
    path_params: BTreeMap<String, String>,
    body: Value,
) -> Result<Json<Value>, NotaryRouteError> {
    let service = state.service().clone();
    let response = service
        .handle(state.request_context(), operation_id, path_params, body)
        .await?;
    Ok(Json(response))
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
