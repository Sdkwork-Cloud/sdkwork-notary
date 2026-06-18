use std::collections::BTreeMap;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use sdkwork_router_notary_http_auth::notary_request_context_from_web;
use sdkwork_web_core::WebRequestContext;
use serde_json::{Map, Value};

use crate::service_port::{NotaryBackendApiState, NotaryRequestContext, NotaryRouteError};

pub async fn list_organization_profiles(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        app_ctx,
        "notary.organizationProfiles.list",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

pub async fn create_organization_profile(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        app_ctx,
        "notary.organizationProfiles.create",
        BTreeMap::new(),
        body,
    )
    .await
}

pub async fn retrieve_organization_profile(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Path(organization_profile_id): Path<String>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_single_param_operation(
        state,
        app_ctx,
        "notary.organizationProfiles.retrieve",
        "organizationProfileId",
        organization_profile_id,
        Value::Null,
    )
    .await
}

pub async fn update_organization_profile(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Path(organization_profile_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_single_param_operation(
        state,
        app_ctx,
        "notary.organizationProfiles.update",
        "organizationProfileId",
        organization_profile_id,
        body,
    )
    .await
}

pub async fn list_matters(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        app_ctx,
        "notary.matters.management.list",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

pub async fn create_matter(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        app_ctx,
        "notary.matters.create",
        BTreeMap::new(),
        body,
    )
    .await
}

pub async fn update_matter(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Path(sku_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_single_param_operation(
        state,
        app_ctx,
        "notary.matters.update",
        "skuId",
        sku_id,
        body,
    )
    .await
}

pub async fn list_cases(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        app_ctx,
        "notary.cases.management.list",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

pub async fn retrieve_case(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_single_param_operation(
        state,
        app_ctx,
        "notary.cases.management.retrieve",
        "caseId",
        case_id,
        Value::Null,
    )
    .await
}

pub async fn create_assignment(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Path(case_id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_single_param_operation(
        state,
        app_ctx,
        "notary.cases.assignments.create",
        "caseId",
        case_id,
        body,
    )
    .await
}

pub async fn delete_assignment(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Path((case_id, assignment_id)): Path<(String, String)>,
) -> Result<Json<Value>, NotaryRouteError> {
    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), case_id);
    path_params.insert("assignmentId".to_string(), assignment_id);
    call_operation(
        state,
        app_ctx,
        "notary.cases.assignments.delete",
        path_params,
        Value::Object(Map::new()),
    )
    .await
}

pub async fn list_staff(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        app_ctx,
        "notary.staff.list",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

pub async fn retrieve_case_summary(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Result<Json<Value>, NotaryRouteError> {
    call_operation(
        state,
        app_ctx,
        "notary.reports.caseSummary.retrieve",
        BTreeMap::new(),
        query_body(query),
    )
    .await
}

async fn call_single_param_operation(
    state: NotaryBackendApiState,
    app_ctx: WebRequestContext,
    operation_id: &'static str,
    param_name: &'static str,
    param_value: String,
    body: Value,
) -> Result<Json<Value>, NotaryRouteError> {
    let mut path_params = BTreeMap::new();
    path_params.insert(param_name.to_string(), param_value);
    call_operation(state, app_ctx, operation_id, path_params, body).await
}

async fn call_operation(
    state: NotaryBackendApiState,
    app_ctx: WebRequestContext,
    operation_id: &'static str,
    path_params: BTreeMap<String, String>,
    body: Value,
) -> Result<Json<Value>, NotaryRouteError> {
    let request_context: NotaryRequestContext = notary_request_context_from_web(&app_ctx)?;
    let service = state.service().clone();
    let response = service
        .handle(request_context, operation_id, path_params, body)
        .await?;
    Ok(Json(response))
}

fn query_body(query: BTreeMap<String, String>) -> Value {
    Value::Object(
        query
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect(),
    )
}
