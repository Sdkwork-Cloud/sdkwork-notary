use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

const NOTARY_BACKEND_API_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/notary/organization_profiles",
        "notary",
        "notary.organizationProfiles.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/backend/v3/api/notary/organization_profiles",
        "notary",
        "notary.organizationProfiles.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/notary/organization_profiles/{organizationProfileId}",
        "notary",
        "notary.organizationProfiles.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Patch,
        "/backend/v3/api/notary/organization_profiles/{organizationProfileId}",
        "notary",
        "notary.organizationProfiles.update",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/notary/matters",
        "notary",
        "notary.matters.management.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/backend/v3/api/notary/matters",
        "notary",
        "notary.matters.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Patch,
        "/backend/v3/api/notary/matters/{skuId}",
        "notary",
        "notary.matters.update",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/notary/cases",
        "notary",
        "notary.cases.management.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/notary/cases/{caseId}",
        "notary",
        "notary.cases.management.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/backend/v3/api/notary/cases/{caseId}/assignments",
        "notary",
        "notary.cases.assignments.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Delete,
        "/backend/v3/api/notary/cases/{caseId}/assignments/{assignmentId}",
        "notary",
        "notary.cases.assignments.delete",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/notary/staff",
        "notary",
        "notary.staff.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/notary/reports/case_summary",
        "notary",
        "notary.reports.caseSummary.retrieve",
    ),
];

pub fn notary_backend_api_http_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(NOTARY_BACKEND_API_ROUTES)
}

pub fn sdkwork_notary_backend_api_route_manifest() -> &'static str {
    r#"{
  "schemaVersion": 1,
  "kind": "sdkwork.route.manifest",
  "packageName": "sdkwork-routes-notary-backend-api",
  "surface": "backend-api",
  "owner": "sdkwork-notary",
  "domain": "notary",
  "capability": "notary",
  "apiAuthority": "sdkwork-notary-backend-api",
  "sdkFamily": "sdkwork-notary-backend-sdk",
  "prefix": "/backend/v3/api",
  "routes": [
    {
      "method": "POST",
      "path": "/backend/v3/api/notary/cases/{caseId}/assignments",
      "operationId": "notary.cases.assignments.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "DELETE",
      "path": "/backend/v3/api/notary/cases/{caseId}/assignments/{assignmentId}",
      "operationId": "notary.cases.assignments.delete",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "GET",
      "path": "/backend/v3/api/notary/cases",
      "operationId": "notary.cases.management.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "GET",
      "path": "/backend/v3/api/notary/cases/{caseId}",
      "operationId": "notary.cases.management.retrieve",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "POST",
      "path": "/backend/v3/api/notary/matters",
      "operationId": "notary.matters.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "GET",
      "path": "/backend/v3/api/notary/matters",
      "operationId": "notary.matters.management.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "PATCH",
      "path": "/backend/v3/api/notary/matters/{skuId}",
      "operationId": "notary.matters.update",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "POST",
      "path": "/backend/v3/api/notary/organization_profiles",
      "operationId": "notary.organizationProfiles.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "GET",
      "path": "/backend/v3/api/notary/organization_profiles",
      "operationId": "notary.organizationProfiles.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "GET",
      "path": "/backend/v3/api/notary/organization_profiles/{organizationProfileId}",
      "operationId": "notary.organizationProfiles.retrieve",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "PATCH",
      "path": "/backend/v3/api/notary/organization_profiles/{organizationProfileId}",
      "operationId": "notary.organizationProfiles.update",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "GET",
      "path": "/backend/v3/api/notary/reports/case_summary",
      "operationId": "notary.reports.caseSummary.retrieve",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    },
    {
      "method": "GET",
      "path": "/backend/v3/api/notary/staff",
      "operationId": "notary.staff.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "backend-api"
    }
  ]
}
"#
}
