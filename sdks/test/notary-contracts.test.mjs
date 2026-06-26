import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const sdksRoot = path.resolve(testDir, "..");
const workspaceRoot = path.resolve(sdksRoot, "..");

const HTTP_METHODS = new Set([
  "get",
  "post",
  "put",
  "patch",
  "delete",
  "head",
  "options",
  "trace",
]);

const ownTables = [
  "notary_organization_profile",
  "notary_case",
  "notary_party",
  "notary_case_assignment",
  "notary_case_event",
];

const forbiddenTables = [
  "notary_staff_profile",
  "notary_matter",
  "notary_order",
  "notary_order_item",
  "notary_document",
  "notary_folder",
  "notary_signature",
];

const crossDomainTables = [
  "commerce_order",
  "commerce_order_item",
  "commerce_product_sku",
  "dr_drive_space",
  "dr_drive_node",
  "iam_organization_membership",
  "iam_role",
  "iam_position",
  "iam_department",
];

const appExpectedOperations = new Map([
  ["/app/v3/api/notary/access", ["get", "notary.access.retrieve"]],
  ["/app/v3/api/notary/matters", ["get", "notary.matters.list"]],
  ["/app/v3/api/notary/staff", ["get", "notary.staff.list"]],
  ["/app/v3/api/notary/cases", ["post", "notary.cases.create"]],
  ["/app/v3/api/notary/cases", ["get", "notary.cases.list"]],
  ["/app/v3/api/notary/cases/{caseId}", ["get", "notary.cases.retrieve"]],
  ["/app/v3/api/notary/cases/{caseId}", ["patch", "notary.cases.update"]],
  [
    "/app/v3/api/notary/cases/{caseId}/assignments",
    ["post", "notary.cases.assignments.create"],
  ],
  [
    "/app/v3/api/notary/cases/{caseId}/acceptances",
    ["post", "notary.cases.acceptances.create"],
  ],
  [
    "/app/v3/api/notary/cases/{caseId}/rejections",
    ["post", "notary.cases.rejections.create"],
  ],
  [
    "/app/v3/api/notary/cases/{caseId}/completions",
    ["post", "notary.cases.completions.create"],
  ],
  ["/app/v3/api/notary/cases/{caseId}/parties", ["get", "notary.cases.parties.list"]],
  ["/app/v3/api/notary/cases/{caseId}/parties", ["post", "notary.cases.parties.create"]],
  [
    "/app/v3/api/notary/cases/{caseId}/parties/{partyId}",
    ["patch", "notary.cases.parties.update"],
  ],
  [
    "/app/v3/api/notary/cases/{caseId}/parties/{partyId}",
    ["delete", "notary.cases.parties.delete"],
  ],
  [
    "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/signatures",
    ["post", "notary.cases.parties.signatures.create"],
  ],
  [
    "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/video_invites",
    ["post", "notary.cases.parties.videoInvites.create"],
  ],
  [
    "/app/v3/api/notary/cases/{caseId}/parties/{partyId}/signature_invites",
    ["post", "notary.cases.parties.signatureInvites.create"],
  ],
  ["/app/v3/api/notary/cases/{caseId}/files", ["get", "notary.cases.files.list"]],
  ["/app/v3/api/notary/cases/{caseId}/files", ["post", "notary.cases.files.create"]],
  [
    "/app/v3/api/notary/cases/{caseId}/download_packages",
    ["post", "notary.cases.downloadPackages.create"],
  ],
  ["/app/v3/api/notary/cases/{caseId}/events", ["get", "notary.cases.events.list"]],
]);

const backendExpectedOperations = new Map([
  ["/backend/v3/api/notary/organization_profiles", ["get", "notary.organizationProfiles.list"]],
  ["/backend/v3/api/notary/organization_profiles", ["post", "notary.organizationProfiles.create"]],
  [
    "/backend/v3/api/notary/organization_profiles/{organizationProfileId}",
    ["get", "notary.organizationProfiles.retrieve"],
  ],
  [
    "/backend/v3/api/notary/organization_profiles/{organizationProfileId}",
    ["patch", "notary.organizationProfiles.update"],
  ],
  ["/backend/v3/api/notary/matters", ["get", "notary.matters.management.list"]],
  ["/backend/v3/api/notary/matters", ["post", "notary.matters.create"]],
  ["/backend/v3/api/notary/matters/{skuId}", ["patch", "notary.matters.update"]],
  ["/backend/v3/api/notary/cases", ["get", "notary.cases.management.list"]],
  ["/backend/v3/api/notary/cases/{caseId}", ["get", "notary.cases.management.retrieve"]],
  [
    "/backend/v3/api/notary/cases/{caseId}/assignments",
    ["post", "notary.cases.assignments.create"],
  ],
  [
    "/backend/v3/api/notary/cases/{caseId}/assignments/{assignmentId}",
    ["delete", "notary.cases.assignments.delete"],
  ],
  ["/backend/v3/api/notary/staff", ["get", "notary.staff.list"]],
  ["/backend/v3/api/notary/reports/case_summary", ["get", "notary.reports.caseSummary.retrieve"]],
]);

const sdkFamilies = [
  {
    root: "sdkwork-notary-app-sdk",
    owner: "sdkwork-notary",
    authority: "sdkwork-notary.app",
    input: "generated/openapi/notary-app-api.openapi.json",
    apiPrefix: "/app/v3/api",
    dependencies: [
      ["sdkwork-iam-app-sdk", "sdkwork-iam-app-api"],
      ["sdkwork-commerce-app-sdk", "sdkwork-commerce.app"],
      ["sdkwork-drive-app-sdk", "sdkwork-drive.app"],
    ],
  },
  {
    root: "sdkwork-notary-backend-sdk",
    owner: "sdkwork-notary",
    authority: "sdkwork-notary.backend",
    input: "generated/openapi/notary-backend-api.openapi.json",
    apiPrefix: "/backend/v3/api",
    dependencies: [
      ["sdkwork-iam-backend-sdk", "sdkwork-iam-backend-api"],
      ["sdkwork-commerce-backend-sdk", "sdkwork-commerce.backend"],
      ["sdkwork-drive-backend-sdk", "sdkwork-drive.backend"],
    ],
  },
];

function readText(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(readText(relativePath));
}

function operationEntries(openapi) {
  const entries = [];
  for (const [pathKey, pathItem] of Object.entries(openapi.paths || {})) {
    for (const [method, operation] of Object.entries(pathItem || {})) {
      if (!HTTP_METHODS.has(method)) {
        continue;
      }
      entries.push({ pathKey, method, operation });
    }
  }
  return entries;
}

function operationCount(openapi) {
  return operationEntries(openapi).length;
}

function requireOperation(openapi, expectedOperations, pathKey) {
  const [method, operationId] = expectedOperations.get(pathKey);
  const operation = openapi.paths?.[pathKey]?.[method];
  assert(operation, `${method.toUpperCase()} ${pathKey} must exist`);
  assert.equal(operation.operationId, operationId);
  return operation;
}

function assertDualTokenOperation(operation, pathKey, apiSurface) {
  assert.deepEqual(
    operation.security,
    [{ AuthToken: [], AccessToken: [] }],
    `${pathKey} must require dual-token auth`,
  );
  assert.equal(operation["x-sdkwork-owner"], "sdkwork-notary");
  assert.equal(operation["x-sdkwork-domain"], "notary");
  assert(operation["x-sdkwork-resource"], `${pathKey} must declare resource metadata`);
  assert(operation["x-sdkwork-permission"], `${pathKey} must declare permission metadata`);
  assert(operation["x-sdkwork-audit-event"], `${pathKey} must declare audit metadata`);
  assert.equal(operation["x-sdkwork-tenant-scope"], "tenant");
  assert.equal(operation["x-sdkwork-data-scope"], "organization");
  assert.equal(
    operation["x-sdkwork-request-context"],
    "WebRequestContext",
    `${pathKey} must declare x-sdkwork-request-context`,
  );
  assert.equal(
    operation["x-sdkwork-api-surface"],
    apiSurface,
    `${pathKey} must declare x-sdkwork-api-surface`,
  );
  assert.equal(
    operation["x-sdkwork-auth-mode"],
    "dual-token",
    `${pathKey} must declare x-sdkwork-auth-mode`,
  );
  assert.equal(
    operation["x-sdkwork-source-route-crate"],
    apiSurface === "app-api"
      ? "sdkwork-routes-notary-app-api"
      : "sdkwork-routes-notary-backend-api",
    `${pathKey} must declare x-sdkwork-source-route-crate`,
  );
  assert(
    typeof operation["x-sdkwork-source"] === "string"
      && operation["x-sdkwork-source"].includes("/src/handlers.rs"),
    `${pathKey} must declare x-sdkwork-source handlers path`,
  );
}

function apiSurfaceForPath(pathKey) {
  if (pathKey.startsWith("/app/v3/")) {
    return "app-api";
  }
  if (pathKey.startsWith("/backend/v3/")) {
    return "backend-api";
  }
  throw new Error(`Unable to infer api surface for ${pathKey}`);
}

function enumValues(openapi, schemaOrRef) {
  if (schemaOrRef?.enum) {
    return schemaOrRef.enum;
  }
  const ref = schemaOrRef?.$ref;
  if (typeof ref === "string" && ref.startsWith("#/components/schemas/")) {
    return openapi.components.schemas[ref.slice("#/components/schemas/".length)]?.enum;
  }
  return undefined;
}

test("notary migration defines the minimal notary-owned table set and reuses existing domains", () => {
  const sql = readText("database/migrations/postgres/0001_notary_foundation.up.sql");
  const normalizedSql = sql.toLowerCase();

  for (const table of ownTables) {
    assert.match(normalizedSql, new RegExp(`create table if not exists\\s+${table}\\b`));
  }

  for (const table of forbiddenTables) {
    assert(!normalizedSql.includes(`create table if not exists ${table}`), `${table} must not exist`);
  }

  for (const reference of crossDomainTables) {
    assert(
      normalizedSql.includes(reference),
      `migration comments or validation notes must document reuse of ${reference}`,
    );
  }

  assert.match(normalizedSql, /drive_space_type text not null default 'notary'/);
  assert.match(normalizedSql, /drive_folder_node_id text not null/);
  assert.match(normalizedSql, /sku_id text not null/);
  assert.match(normalizedSql, /order_id text not null/);
  assert.match(normalizedSql, /order_item_id text not null/);
  assert.match(normalizedSql, /identity_no_hash text not null/);
  assert.match(normalizedSql, /identity_no_encrypted text not null/);
  assert.match(normalizedSql, /idx_notary_case_drive_folder/);
  assert.match(normalizedSql, /idx_notary_party_identity_hash/);
  assert.match(normalizedSql, /unique \(tenant_id, order_item_id\)/);
});

test("notary schema registry mirrors the SQL table contract", () => {
  const registry = readText("docs/schema-registry/tables/001-notary-core.yaml");

  for (const table of ownTables) {
    assert(registry.includes(`table: ${table}`), `${table} must be registered`);
  }

  assert(registry.includes("system_of_record: true"));
  assert(registry.includes("write_owner: sdkwork-notary"));
  assert(registry.includes("compliance_level: L3"));
  assert(registry.includes("references: commerce_order.id"));
  assert(registry.includes("references: commerce_order_item.id"));
  assert(registry.includes("references: commerce_product_sku.id"));
  assert(registry.includes("references: dr_drive_space.id"));
  assert(registry.includes("references: dr_drive_node.id"));
  assert(registry.includes("references: iam_organization_membership.id"));
  assert(registry.includes("encrypted_fields: [identity_no_encrypted, phone_encrypted, address_encrypted]"));
});

test("notary app OpenAPI is owner-only, dual-token protected, and aligned to frontend workflow", () => {
  const openapi = readJson("generated/openapi/notary-app-api.openapi.json");
  assert.equal(openapi.openapi, "3.1.2");
  assert.equal(openapi["x-sdkwork-owner"], "sdkwork-notary");
  assert.equal(openapi["x-sdkwork-api-authority"], "sdkwork-notary.app");
  assert.equal(openapi.components.securitySchemes.AuthToken.scheme, "bearer");
  assert.equal(openapi.components.securitySchemes.AccessToken.name, "Access-Token");

  for (const pathKey of appExpectedOperations.keys()) {
    assertDualTokenOperation(
      requireOperation(openapi, appExpectedOperations, pathKey),
      pathKey,
      "app-api",
    );
  }

  const createCase = openapi.paths["/app/v3/api/notary/cases"].post;
  assert.equal(createCase["x-sdkwork-idempotent"], true);
  assert.equal(
    createCase.requestBody.content["application/json"].schema.$ref,
    "#/components/schemas/CreateNotaryCaseRequest",
  );
  const createCaseRequest = openapi.components.schemas.CreateNotaryCaseRequest;
  assert(
    createCaseRequest.properties.primaryNotaryMembershipId,
    "CreateNotaryCaseRequest must accept selected Organization member as primaryNotaryMembershipId",
  );
  assert(
    createCaseRequest.properties.driveFolderName,
    "CreateNotaryCaseRequest must accept the frontend Drive folder name",
  );

  const caseSchema = openapi.components.schemas.NotaryCase;
  for (const field of [
    "orderId",
    "orderItemId",
    "skuId",
    "driveSpaceId",
    "driveSpaceType",
    "driveFolderNodeId",
    "parties",
    "documents",
    "timeline",
  ]) {
    assert(caseSchema.required.includes(field), `NotaryCase must require ${field}`);
  }

  assert.deepEqual(caseSchema.properties.driveSpaceType.enum, ["notary"]);
  assert.deepEqual(enumValues(openapi, caseSchema.properties.status), [
    "PENDING_REVIEW",
    "PROCESSING",
    "COMPLETED",
    "REJECTED",
    "CANCELLED",
    "CREATE_FAILED",
  ]);

  const videoInvite = openapi.components.schemas.PartyVideoInvite;
  assert(videoInvite.required.includes("inviteId"));
  assert(videoInvite.required.includes("caseId"));
  assert(videoInvite.required.includes("partyId"));
  assert(videoInvite.required.includes("conversationId"));
  assert(videoInvite.required.includes("inviteUrl"));
  assert(videoInvite.required.includes("expiresAt"));
  assert.deepEqual(videoInvite.properties.driveSpaceType.enum, ["notary"]);

  const signatureInviteRequest = openapi.components.schemas.CreatePartySignatureInviteRequest;
  assert.deepEqual(signatureInviteRequest.properties.purpose.enum, [
    "remote_signature",
    "onsite_signature_confirmation",
    "material_signature",
  ]);

  const signatureInvite = openapi.components.schemas.PartySignatureInvite;
  assert(signatureInvite.required.includes("inviteId"));
  assert(signatureInvite.required.includes("caseId"));
  assert(signatureInvite.required.includes("partyId"));
  assert(signatureInvite.required.includes("inviteUrl"));
  assert(signatureInvite.required.includes("signingUrl"));
  assert(signatureInvite.required.includes("expiresAt"));
  assert(signatureInvite.required.includes("driveSpaceId"));
  assert(signatureInvite.required.includes("driveSpaceType"));
  assert(signatureInvite.required.includes("driveFolderNodeId"));
  assert.deepEqual(signatureInvite.properties.driveSpaceType.enum, ["notary"]);

  const staff = openapi.components.schemas.NotaryStaffMember;
  for (const field of [
    "membershipId",
    "userId",
    "displayName",
    "status",
    "roles",
    "positions",
    "departments",
  ]) {
    assert(staff.required.includes(field), `NotaryStaffMember must require ${field}`);
  }
  assert.deepEqual(staff.properties.notaryStaffRole.enum, [
    "notary",
    "assistant",
    "reviewer",
    "approver",
  ]);

  const assignmentRequest = openapi.components.schemas.CreateCaseAssignmentRequest;
  assert.deepEqual(assignmentRequest.required, ["organizationMembershipId", "assignmentRole"]);
  assert.deepEqual(assignmentRequest.properties.assignmentRole.enum, [
    "primary_notary",
    "assistant",
    "reviewer",
    "approver",
  ]);

  const assignment = openapi.components.schemas.NotaryCaseAssignment;
  for (const field of [
    "id",
    "caseId",
    "organizationMembershipId",
    "userId",
    "assignmentRole",
    "status",
    "assignedAt",
  ]) {
    assert(assignment.required.includes(field), `NotaryCaseAssignment must require ${field}`);
  }
});

test("notary backend OpenAPI manages profiles, SKU-backed matters, IAM staff view, and assignments", () => {
  const openapi = readJson("generated/openapi/notary-backend-api.openapi.json");
  assert.equal(openapi.openapi, "3.1.2");
  assert.equal(openapi["x-sdkwork-owner"], "sdkwork-notary");
  assert.equal(openapi["x-sdkwork-api-authority"], "sdkwork-notary.backend");

  for (const pathKey of backendExpectedOperations.keys()) {
    assertDualTokenOperation(
      requireOperation(openapi, backendExpectedOperations, pathKey),
      pathKey,
      "backend-api",
    );
  }

  const matter = openapi.components.schemas.NotaryMatter;
  assert(matter.required.includes("skuId"));
  assert(matter.required.includes("spuId"));
  assert(matter.required.includes("priceAmount"));
  assert(!openapi.paths["/backend/v3/api/auth"], "backend API must not expose auth namespace");
});

test("authored apis contracts materialize to generated openapi authorities", () => {
  const pairs = [
    [
      "apis/app-api/notary/notary-app-api.openapi.json",
      "generated/openapi/notary-app-api.openapi.json",
    ],
    [
      "apis/backend-api/notary/notary-backend-api.openapi.json",
      "generated/openapi/notary-backend-api.openapi.json",
    ],
  ];

  for (const [source, target] of pairs) {
    assert.deepEqual(readJson(source), readJson(target), `${target} must match ${source}`);
  }
});

test("notary OpenAPI and generated SDKs do not expose client-writable tenant context", () => {
  const forbiddenParameterNames = new Set(["tenantid", "tenant_id"]);
  const openapiFiles = [
    "apis/app-api/notary/notary-app-api.openapi.json",
    "apis/backend-api/notary/notary-backend-api.openapi.json",
    "generated/openapi/notary-app-api.openapi.json",
    "generated/openapi/notary-backend-api.openapi.json",
  ];

  for (const file of openapiFiles) {
    const openapi = readJson(file);
    for (const { pathKey, method, operation } of operationEntries(openapi)) {
      for (const parameter of operation.parameters ?? []) {
        const name = String(parameter.name ?? parameter.$ref ?? "").toLowerCase();
        assert.equal(
          forbiddenParameterNames.has(name),
          false,
          `${file} ${method.toUpperCase()} ${pathKey} must not accept client-writable ${parameter.name}`,
        );
      }
    }
  }

  for (const generatedApiDir of [
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/generated/server-openapi/src/api",
    "sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/generated/server-openapi/src/api",
  ]) {
    const apiDir = path.join(workspaceRoot, generatedApiDir);
    if (!existsSync(apiDir)) {
      continue;
    }
    for (const fileName of readdirSync(apiDir).filter((name) => name.endsWith(".ts"))) {
      const source = readText(path.join(generatedApiDir, fileName));
      assert.doesNotMatch(
        source,
        /name:\s*['"]tenantId['"]/,
        `${generatedApiDir}/${fileName} must not expose tenantId params`,
      );
      assert.doesNotMatch(
        source,
        /name:\s*['"]tenant_id['"]/,
        `${generatedApiDir}/${fileName} must not expose tenant_id params`,
      );
    }
  }
});

test("every notary OpenAPI operation declares web-framework contract extensions", () => {
  const expectations = [
    { file: "generated/openapi/notary-app-api.openapi.json", apiSurface: "app-api" },
    { file: "generated/openapi/notary-backend-api.openapi.json", apiSurface: "backend-api" },
  ];

  for (const { file, apiSurface } of expectations) {
    const openapi = readJson(file);
    for (const { pathKey, operation } of operationEntries(openapi)) {
      assert.equal(
        operation["x-sdkwork-request-context"],
        "WebRequestContext",
        `${file} ${pathKey} missing x-sdkwork-request-context`,
      );
      assert.equal(
        operation["x-sdkwork-api-surface"],
        apiSurface,
        `${file} ${pathKey} missing x-sdkwork-api-surface`,
      );
      assert.equal(
        operation["x-sdkwork-auth-mode"],
        "dual-token",
        `${file} ${pathKey} missing x-sdkwork-auth-mode`,
      );
      assert.ok(
        operation["x-sdkwork-source-route-crate"],
        `${file} ${pathKey} missing x-sdkwork-source-route-crate`,
      );
      assert.ok(
        operation["x-sdkwork-source"],
        `${file} ${pathKey} missing x-sdkwork-source`,
      );
      assert.equal(apiSurfaceForPath(pathKey), apiSurface, `${pathKey} api surface mismatch`);
    }
  }
});

test("notary SDK family manifests declare dependency SDKs without copying dependency operations", () => {
  for (const family of sdkFamilies) {
    const assembly = readJson(path.join("sdks", family.root, ".sdkwork-assembly.json"));
    const manifest = readJson(path.join("sdks", family.root, "sdk-manifest.json"));
    const componentSpec = readJson(path.join("sdks", family.root, "specs/component.spec.json"));
    const openapi = readJson(family.input);

    assert.equal(assembly.sdkOwner, family.owner);
    assert.equal(assembly.apiAuthority, family.authority);
    assert.equal(assembly.sdkFamily, family.root);
    assert.equal(manifest.sdkName, family.root);
    assert.equal(manifest.sdkFamily, family.root);
    assert.equal(manifest.apiPrefix, family.apiPrefix);
    assert.equal(manifest.standardProfile, "sdkwork-v3");
    assert.equal(manifest.generationInputSpec, `../../${family.input}`);
    assert.equal(manifest.ownerOnlyOperationCount, operationCount(openapi));
    assert.deepEqual(assembly.sdkDependencies, manifest.sdkDependencies);
    assert.deepEqual(componentSpec.contracts.sdkDependencies, manifest.sdkDependencies);
    assert.deepEqual(assembly.dependencyApiExports, []);
    assert.deepEqual(manifest.dependencyApiExports, []);
    assert.deepEqual(componentSpec.contracts.dependencyApiExports, []);

    assert.deepEqual(
      manifest.sdkDependencies.map((dependency) => [
        dependency.workspace,
        dependency.apiAuthority,
        dependency.dependencyMode,
        dependency.generatedTransportImportPolicy,
      ]),
      family.dependencies.map(([workspace, authority]) => [
        workspace,
        authority,
        "consumer-sdk",
        "forbidden",
      ]),
    );

    for (const { pathKey, operation } of operationEntries(openapi)) {
      assert(pathKey.startsWith(family.apiPrefix), `${pathKey} must use ${family.apiPrefix}`);
      assert.equal(operation["x-sdkwork-owner"], "sdkwork-notary");
      assert.equal(operation["x-sdkwork-api-authority"], family.authority);
      assert(!pathKey.includes("/drive/"), `${pathKey} must not copy Drive API routes`);
      assert(!pathKey.includes("/commerce/"), `${pathKey} must not copy Commerce API routes`);
      assert(!pathKey.includes("/iam/"), `${pathKey} must not copy Appbase/IAM API routes`);
    }
  }
});

test("notary generated TypeScript SDK output is present and remains generator-owned", () => {
  for (const family of sdkFamilies) {
    const generatedRoot = path.join(
      "sdks",
      family.root,
      `${family.root}-typescript`,
      "generated",
      "server-openapi",
    );
    const sdkMetadata = readJson(path.join(generatedRoot, "sdkwork-sdk.json"));
    const packageManifest = readJson(path.join(generatedRoot, "package.json"));
    const sourceOpenapi = readJson(path.join(generatedRoot, "source-openapi.json"));

    assert.equal(sdkMetadata.name, family.root);
    assert.equal(packageManifest.name, `${family.root}-generated-typescript`);
    assert.equal(sourceOpenapi["x-sdkwork-owner"], family.owner);
    assert.equal(sourceOpenapi["x-sdkwork-api-authority"], family.authority);

    const generatedMetadataText = [
      readText(path.join(generatedRoot, "sdkwork-sdk.json")),
      readText(path.join(generatedRoot, "package.json")),
      readText(path.join(generatedRoot, "source-openapi.json")),
    ].join("\n");
    assert(!generatedMetadataText.includes('"sdkDependencies"'));
    assert(!generatedMetadataText.includes('"dependencyApiExports"'));
    assert(existsSync(path.join(workspaceRoot, generatedRoot, ".sdkwork", "sdkwork-generator-manifest.json")));
    assert(existsSync(path.join(workspaceRoot, generatedRoot, "custom", "README.md")));
  }
});

test("notary TypeScript composed facades expose injected dependency ports and no raw HTTP", () => {
  const appFacadePath = "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts";
  const backendFacadePath =
    "sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/composed/index.ts";

  assert(existsSync(path.join(workspaceRoot, appFacadePath)), "app composed facade must exist");
  assert(existsSync(path.join(workspaceRoot, backendFacadePath)), "backend composed facade must exist");

  for (const relativePath of [appFacadePath, backendFacadePath]) {
    const source = readText(relativePath);
    assert(!source.includes("fetch("), `${relativePath} must not use raw fetch`);
    assert(!source.includes("axios"), `${relativePath} must not use axios`);
    assert(!source.includes("Authorization"), `${relativePath} must not assemble auth headers`);
    assert(!source.includes("Access-Token"), `${relativePath} must not assemble access headers`);
    assert(source.includes("notary"), `${relativePath} must inject notary SDK client`);
    assert(source.includes("drive"), `${relativePath} must inject drive SDK client`);
    assert(source.includes("commerce"), `${relativePath} must inject commerce SDK client`);
    assert(source.includes("appbase"), `${relativePath} must inject appbase SDK client`);
  }

  const appSource = readText(appFacadePath);
  assert(appSource.includes("createNotaryApi"));
  assert(appSource.includes("createCase"));
  assert(appSource.includes("listCaseFiles"));
  assert(appSource.includes("uploadCaseFile"));
  assert(appSource.includes("driveSpaceType: \"notary\""));

  const backendSource = readText(backendFacadePath);
  assert(backendSource.includes("createNotaryBackendApi"));
  assert(backendSource.includes("createMatterSku"));
  assert(backendSource.includes("listStaffMembers"));
  assert(
    !backendSource.includes("drive.spaces.create"),
    "backend composed facade must let notary backend runtime create the NOTARY Drive space exactly once",
  );
});
