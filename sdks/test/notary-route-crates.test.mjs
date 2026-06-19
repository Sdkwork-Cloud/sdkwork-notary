import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");

const routeCrates = [
  {
    root: "crates/sdkwork-router-notary-app-api",
    packageName: "sdkwork-router-notary-app-api",
    builder: "build_sdkwork_notary_app_api_router",
    manifestFn: "sdkwork_notary_app_api_route_manifest",
    surface: "app-api",
    prefix: "/app/v3/api",
    authority: "sdkwork-notary-app-api",
    sdkFamily: "sdkwork-notary-app-sdk",
    openapi: "generated/openapi/notary-app-api.openapi.json",
    expectedPaths: [
      "/app/v3/api/notary/access",
      "/app/v3/api/notary/matters",
      "/app/v3/api/notary/staff",
      "/app/v3/api/notary/cases",
      "/app/v3/api/notary/cases/:case_id",
      "/app/v3/api/notary/cases/:case_id/assignments",
      "/app/v3/api/notary/cases/:case_id/files",
      "/app/v3/api/notary/cases/:case_id/events",
      "/app/v3/api/notary/cases/:case_id/parties/:party_id/video_invites",
      "/app/v3/api/notary/cases/:case_id/parties/:party_id/signature_invites",
    ],
  },
  {
    root: "crates/sdkwork-router-notary-backend-api",
    packageName: "sdkwork-router-notary-backend-api",
    builder: "build_sdkwork_notary_backend_api_router",
    manifestFn: "sdkwork_notary_backend_api_route_manifest",
    surface: "backend-api",
    prefix: "/backend/v3/api",
    authority: "sdkwork-notary-backend-api",
    sdkFamily: "sdkwork-notary-backend-sdk",
    openapi: "generated/openapi/notary-backend-api.openapi.json",
    expectedPaths: [
      "/backend/v3/api/notary/organization_profiles",
      "/backend/v3/api/notary/matters",
      "/backend/v3/api/notary/cases",
      "/backend/v3/api/notary/staff",
      "/backend/v3/api/notary/reports/case_summary",
    ],
  },
];

function readText(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(readText(relativePath));
}

function openApiOperations(relativePath) {
  const openapi = readJson(relativePath);
  const operations = [];
  for (const [pathKey, pathItem] of Object.entries(openapi.paths || {})) {
    for (const [method, operation] of Object.entries(pathItem || {})) {
      if (operation?.operationId) {
        operations.push({
          method: method.toUpperCase(),
          pathKey,
          operationId: operation.operationId,
        });
      }
    }
  }
  return operations.sort((left, right) => left.operationId.localeCompare(right.operationId));
}

test("notary Rust route crates are workspace members with standard component specs", () => {
  const workspaceManifest = readText("Cargo.toml");

  for (const routeCrate of routeCrates) {
    assert(
      workspaceManifest.includes(`"${routeCrate.root}"`),
      `${routeCrate.packageName} must be a Cargo workspace member`,
    );
    assert(!workspaceManifest.includes("packages/native-rust"));
    assert(!workspaceManifest.includes("sdkwork-routes-notary"));
    for (const relativePath of [
      "Cargo.toml",
      "README.md",
      "src/lib.rs",
      "src/paths.rs",
      "src/routes.rs",
      "src/handlers.rs",
      "src/manifest.rs",
      "src/service_port.rs",
      "specs/component.spec.json",
    ]) {
      assert(
        existsSync(path.join(workspaceRoot, routeCrate.root, relativePath)),
        `${routeCrate.packageName} missing ${relativePath}`,
      );
    }

    const spec = readJson(path.join(routeCrate.root, "specs/component.spec.json"));
    assert.equal(spec.kind, "sdkwork.component.spec");
    assert.equal(spec.component.name, routeCrate.packageName);
    assert.equal(spec.component.type, "rust-route-crate");
    assert.equal(spec.contracts.routeManifest, `../../../sdks/_route-manifests/${routeCrate.surface}/sdkwork-router-${routeCrate.surface === "app-api" ? "notary-app-api" : "notary-backend-api"}.route-manifest.json`);
    assert(spec.contracts.runtimeEntrypoints.includes(routeCrate.builder));
  }
});

test("notary route manifests cover every OpenAPI operation", () => {
  for (const routeCrate of routeCrates) {
    const manifest = readText(path.join(routeCrate.root, "src/manifest.rs"));

    for (const operation of openApiOperations(routeCrate.openapi)) {
      assert(
        manifest.includes(`"method": "${operation.method}"`),
        `${routeCrate.packageName} manifest must include ${operation.method}`,
      );
      assert(
        manifest.includes(`"path": "${operation.pathKey}"`),
        `${routeCrate.packageName} manifest must include ${operation.pathKey}`,
      );
      assert(
        manifest.includes(`"operationId": "${operation.operationId}"`),
        `${routeCrate.packageName} manifest must include ${operation.operationId}`,
      );
    }
  }
});

test("notary route crates expose executable routers and deterministic route manifests", () => {
  for (const routeCrate of routeCrates) {
    const lib = readText(path.join(routeCrate.root, "src/lib.rs"));
    const routes = readText(path.join(routeCrate.root, "src/routes.rs"));
    const manifest = readText(path.join(routeCrate.root, "src/manifest.rs"));
    const paths = readText(path.join(routeCrate.root, "src/paths.rs"));

    assert(lib.includes(routeCrate.builder));
    assert(routes.includes(routeCrate.builder));
    assert(manifest.includes(routeCrate.manifestFn));
    assert(manifest.includes("sdkwork.route.manifest"));
    assert(manifest.includes(routeCrate.packageName));
    assert(manifest.includes(routeCrate.authority));
    assert(manifest.includes(routeCrate.sdkFamily));

    for (const pathKey of routeCrate.expectedPaths) {
      assert(paths.includes(pathKey), `${routeCrate.packageName} must declare ${pathKey}`);
      assert(routes.includes(pathKey), `${routeCrate.packageName} must mount ${pathKey}`);
    }
  }
});

test("notary canonical route manifests stay aligned with embedded crate manifests", () => {
  for (const routeCrate of routeCrates) {
    const manifestFile = readJson(
      `sdks/_route-manifests/${routeCrate.surface}/sdkwork-router-${routeCrate.surface === "app-api" ? "notary-app-api" : "notary-backend-api"}.route-manifest.json`,
    );
    const embedded = JSON.parse(
      readText(path.join(routeCrate.root, "src/manifest.rs")).match(
        /pub fn sdkwork_notary_[a-z_]+_route_manifest\(\) -> &'static str \{\s*r#"([\s\S]*?)"#/,
      )[1],
    );

    const fileRoutes = manifestFile.routes
      .map((route) => `${route.method} ${route.path} ${route.operationId}`)
      .sort();
    const embeddedRoutes = embedded.routes
      .map((route) => `${route.method} ${route.path} ${route.operationId}`)
      .sort();

    assert.deepEqual(embeddedRoutes, fileRoutes, `${routeCrate.packageName} embedded manifest drift`);
    assert.equal(manifestFile.routes.length, openApiOperations(routeCrate.openapi).length);
  }
});

test("notary route manifests mirror OpenAPI auth and permission metadata", () => {
  for (const routeCrate of routeCrates) {
    const manifestFile = readJson(
      `sdks/_route-manifests/${routeCrate.surface}/sdkwork-router-${routeCrate.surface === "app-api" ? "notary-app-api" : "notary-backend-api"}.route-manifest.json`,
    );
    const openapi = readJson(routeCrate.openapi);
    const operationsById = new Map(
      openApiOperations(routeCrate.openapi).map((operation) => [operation.operationId, operation]),
    );

    for (const route of manifestFile.routes) {
      const operation = operationsById.get(route.operationId);
      assert(operation, `${route.operationId} must exist in ${routeCrate.openapi}`);
      const openapiOperation = openapi.paths[operation.pathKey]?.[operation.method.toLowerCase()];
      assert(openapiOperation, `${route.operationId} OpenAPI operation must exist`);
      assert.equal(route.auth?.mode, "dual-token", `${route.operationId} auth.mode`);
      assert.equal(
        route.auth?.permission,
        openapiOperation["x-sdkwork-permission"],
        `${route.operationId} permission drift`,
      );
      assert.equal(
        route.ownership?.apiAuthority,
        manifestFile.apiAuthority,
        `${route.operationId} ownership.apiAuthority`,
      );
    }
  }
});

test("notary route handlers use injected service ports and do not bypass SDKWork boundaries", () => {
  for (const routeCrate of routeCrates) {
    const source = [
      "src/handlers.rs",
      "src/routes.rs",
      "src/service_port.rs",
      "src/manifest.rs",
    ]
      .map((relativePath) => readText(path.join(routeCrate.root, relativePath)))
      .join("\n");

    assert(source.includes("WebRequestContext"));
    assert(source.includes("notary_request_context_from_web"));
    assert(source.includes("with_dual_token_request_context"));
    assert(source.includes("State(state)"));
    assert(source.includes(".service()."));
    assert(!source.includes("fetch("));
    assert(!source.includes("axios"));
    assert(!source.includes("Authorization"));
    assert(!source.includes("Access-Token"));
    assert(!source.includes("X-API-Key"));
    assert(!source.includes("sdkwork_notary_app_sdk"));
    assert(!source.includes("sdkwork_notary_backend_sdk"));
    assert(!source.includes("commerce_"));
    assert(!source.includes("dr_drive_"));
    assert(!source.includes("iam_"));
  }
});
