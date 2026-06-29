use std::collections::BTreeMap;

use axum::{
    extract::{Path, Query, State},
    response::Response,
    Json,
};
use sdkwork_routes_notary_http_auth::{
    finish_success, notary_request_context_from_web, success_status_for_notary_backend_operation,
};
use sdkwork_web_core::WebRequestContext;
use serde_json::{Map, Value};

use crate::service_port::NotaryBackendApiState;

pub async fn list_organization_profiles(
    State(state): State<NotaryBackendApiState>,
    app_ctx: WebRequestContext,
    Query(query): Query<BTreeMap<String, String>>,
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
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
) -> Response {
    let result = async {
        let request_context = notary_request_context_from_web(&app_ctx)?;
        let service = state.service().clone();
        let response = service
            .handle(request_context, operation_id, path_params, body)
            .await?;
        finish_success(
            &app_ctx,
            success_status_for_notary_backend_operation(operation_id),
            response,
        )
    }
    .await;

    match result {
        Ok(response) => response,
        Err(error) => error.into_response_for(&app_ctx),
    }
}

fn query_body(query: BTreeMap<String, String>) -> Value {
    Value::Object(
        query
            .into_iter()
            .map(|(key, value)| (key, Value::String(value)))
            .collect(),
    )
}
