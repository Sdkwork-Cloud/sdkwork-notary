pub fn sdkwork_notary_backend_api_route_manifest() -> &'static str {
    r#"{
  "schemaVersion": 1,
  "kind": "sdkwork.route.manifest",
  "packageName": "sdkwork-router-notary-backend-api",
  "surface": "backend-api",
  "owner": "sdkwork-notary",
  "domain": "notary",
  "capability": "notary",
  "apiAuthority": "sdkwork-notary-backend-api",
  "sdkFamily": "sdkwork-notary-backend-sdk",
  "prefix": "/backend/v3/api",
  "routes": [
    {"method": "GET", "path": "/backend/v3/api/notary/organization_profiles", "operationId": "notary.organizationProfiles.list"},
    {"method": "POST", "path": "/backend/v3/api/notary/organization_profiles", "operationId": "notary.organizationProfiles.create"},
    {"method": "GET", "path": "/backend/v3/api/notary/organization_profiles/{organizationProfileId}", "operationId": "notary.organizationProfiles.retrieve"},
    {"method": "PATCH", "path": "/backend/v3/api/notary/organization_profiles/{organizationProfileId}", "operationId": "notary.organizationProfiles.update"},
    {"method": "GET", "path": "/backend/v3/api/notary/matters", "operationId": "notary.matters.management.list"},
    {"method": "POST", "path": "/backend/v3/api/notary/matters", "operationId": "notary.matters.create"},
    {"method": "PATCH", "path": "/backend/v3/api/notary/matters/{skuId}", "operationId": "notary.matters.update"},
    {"method": "GET", "path": "/backend/v3/api/notary/cases", "operationId": "notary.cases.management.list"},
    {"method": "GET", "path": "/backend/v3/api/notary/cases/{caseId}", "operationId": "notary.cases.management.retrieve"},
    {"method": "POST", "path": "/backend/v3/api/notary/cases/{caseId}/assignments", "operationId": "notary.cases.assignments.create"},
    {"method": "DELETE", "path": "/backend/v3/api/notary/cases/{caseId}/assignments/{assignmentId}", "operationId": "notary.cases.assignments.delete"},
    {"method": "GET", "path": "/backend/v3/api/notary/staff", "operationId": "notary.staff.list"},
    {"method": "GET", "path": "/backend/v3/api/notary/reports/case_summary", "operationId": "notary.reports.caseSummary.retrieve"}
  ]
}"#
}
