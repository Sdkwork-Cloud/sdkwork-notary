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
    {"method": "GET", "path": "/app/v3/api/notary/access", "operationId": "notary.access.retrieve"},
    {"method": "GET", "path": "/app/v3/api/notary/matters", "operationId": "notary.matters.list"},
    {"method": "GET", "path": "/app/v3/api/notary/staff", "operationId": "notary.staff.list"},
    {"method": "GET", "path": "/app/v3/api/notary/cases", "operationId": "notary.cases.list"},
    {"method": "POST", "path": "/app/v3/api/notary/cases", "operationId": "notary.cases.create"},
    {"method": "GET", "path": "/app/v3/api/notary/cases/{caseId}", "operationId": "notary.cases.retrieve"},
    {"method": "PATCH", "path": "/app/v3/api/notary/cases/{caseId}", "operationId": "notary.cases.update"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/acceptances", "operationId": "notary.cases.acceptances.create"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/rejections", "operationId": "notary.cases.rejections.create"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/completions", "operationId": "notary.cases.completions.create"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/assignments", "operationId": "notary.cases.assignments.create"},
    {"method": "GET", "path": "/app/v3/api/notary/cases/{caseId}/files", "operationId": "notary.cases.files.list"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/files", "operationId": "notary.cases.files.create"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/download_packages", "operationId": "notary.cases.downloadPackages.create"},
    {"method": "GET", "path": "/app/v3/api/notary/cases/{caseId}/parties", "operationId": "notary.cases.parties.list"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/parties", "operationId": "notary.cases.parties.create"},
    {"method": "PATCH", "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}", "operationId": "notary.cases.parties.update"},
    {"method": "DELETE", "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}", "operationId": "notary.cases.parties.delete"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/signatures", "operationId": "notary.cases.parties.signatures.create"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/video_invites", "operationId": "notary.cases.parties.videoInvites.create"},
    {"method": "POST", "path": "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/signature_invites", "operationId": "notary.cases.parties.signatureInvites.create"},
    {"method": "GET", "path": "/app/v3/api/notary/cases/{caseId}/events", "operationId": "notary.cases.events.list"}
  ]
}"#
}
