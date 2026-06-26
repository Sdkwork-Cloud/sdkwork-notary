use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

const NOTARY_APP_API_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/access",
        "notary",
        "notary.access.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/dashboard/statistics",
        "notary",
        "notary.dashboard.statistics.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/matters",
        "notary",
        "notary.matters.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/reports/monthly",
        "notary",
        "notary.reports.monthly.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/staff",
        "notary",
        "notary.staff.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/cases",
        "notary",
        "notary.cases.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases",
        "notary",
        "notary.cases.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/cases/{caseId}",
        "notary",
        "notary.cases.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Patch,
        "/app/v3/api/notary/cases/{caseId}",
        "notary",
        "notary.cases.update",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/acceptances",
        "notary",
        "notary.cases.acceptances.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/rejections",
        "notary",
        "notary.cases.rejections.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/completions",
        "notary",
        "notary.cases.completions.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/assignments",
        "notary",
        "notary.cases.assignments.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/cases/{caseId}/files",
        "notary",
        "notary.cases.files.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/files",
        "notary",
        "notary.cases.files.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/download_packages",
        "notary",
        "notary.cases.downloadPackages.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/cases/{caseId}/parties",
        "notary",
        "notary.cases.parties.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/parties",
        "notary",
        "notary.cases.parties.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Patch,
        "/app/v3/api/notary/cases/{caseId}/parties/{partyId}",
        "notary",
        "notary.cases.parties.update",
    ),
    HttpRoute::dual_token(
        HttpMethod::Delete,
        "/app/v3/api/notary/cases/{caseId}/parties/{partyId}",
        "notary",
        "notary.cases.parties.delete",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/signatures",
        "notary",
        "notary.cases.parties.signatures.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/video_invites",
        "notary",
        "notary.cases.parties.videoInvites.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/signature_invites",
        "notary",
        "notary.cases.parties.signatureInvites.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/notary/cases/{caseId}/events",
        "notary",
        "notary.cases.events.list",
    ),
];

pub fn notary_app_api_http_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(NOTARY_APP_API_ROUTES)
}

pub fn sdkwork_notary_app_api_route_manifest() -> &'static str {
    r#"{
  "schemaVersion": 1,
  "kind": "sdkwork.route.manifest",
  "packageName": "sdkwork-routes-notary-app-api",
  "surface": "app-api",
  "owner": "sdkwork-notary",
  "domain": "notary",
  "capability": "notary",
  "apiAuthority": "sdkwork-notary-app-api",
  "sdkFamily": "sdkwork-notary-app-sdk",
  "prefix": "/app/v3/api",
  "routes": [
    {
      "method": "GET",
      "path": "/app/v3/api/notary/access",
      "operationId": "notary.access.retrieve",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/acceptances",
      "operationId": "notary.cases.acceptances.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/assignments",
      "operationId": "notary.cases.assignments.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/completions",
      "operationId": "notary.cases.completions.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases",
      "operationId": "notary.cases.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/download_packages",
      "operationId": "notary.cases.downloadPackages.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/cases/{caseId}/events",
      "operationId": "notary.cases.events.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/files",
      "operationId": "notary.cases.files.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/cases/{caseId}/files",
      "operationId": "notary.cases.files.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/cases",
      "operationId": "notary.cases.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/parties",
      "operationId": "notary.cases.parties.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "DELETE",
      "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}",
      "operationId": "notary.cases.parties.delete",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/cases/{caseId}/parties",
      "operationId": "notary.cases.parties.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/signature_invites",
      "operationId": "notary.cases.parties.signatureInvites.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/signatures",
      "operationId": "notary.cases.parties.signatures.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "PATCH",
      "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}",
      "operationId": "notary.cases.parties.update",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/video_invites",
      "operationId": "notary.cases.parties.videoInvites.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "POST",
      "path": "/app/v3/api/notary/cases/{caseId}/rejections",
      "operationId": "notary.cases.rejections.create",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/cases/{caseId}",
      "operationId": "notary.cases.retrieve",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "PATCH",
      "path": "/app/v3/api/notary/cases/{caseId}",
      "operationId": "notary.cases.update",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/dashboard/statistics",
      "operationId": "notary.dashboard.statistics.retrieve",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/matters",
      "operationId": "notary.matters.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/reports/monthly",
      "operationId": "notary.reports.monthly.retrieve",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    },
    {
      "method": "GET",
      "path": "/app/v3/api/notary/staff",
      "operationId": "notary.staff.list",
      "requestContext": "WebRequestContext",
      "apiSurface": "app-api"
    }
  ]
}
"#
}
