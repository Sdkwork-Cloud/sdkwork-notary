import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");

function readJson(relativePath) {
  return JSON.parse(readFileSync(path.join(workspaceRoot, relativePath), "utf8"));
}

function readText(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), "utf8");
}

function openApiOperationIds(relativePath) {
  const openapi = readJson(relativePath);
  const operations = [];
  for (const pathItem of Object.values(openapi.paths || {})) {
    for (const [method, operation] of Object.entries(pathItem || {})) {
      if (operation?.operationId) {
        operations.push({
          method: method.toUpperCase(),
          operationId: operation.operationId,
        });
      }
    }
  }
  return operations.sort((left, right) => left.operationId.localeCompare(right.operationId));
}

function notaryRuntimeContractSource() {
  const service = readText("crates/sdkwork-notary-case-service/src/service.rs");
  return sourceSlice(
    service,
    "pub fn notary_runtime_contract()",
    "pub async fn ensure_notary_business_open",
    "notary_runtime_contract",
  );
}

function sourceSlice(source, startNeedle, endNeedle, label) {
  const start = source.indexOf(startNeedle);
  const end = source.indexOf(endNeedle, start + startNeedle.length);
  assert(start > -1, `${label} must exist`);
  assert(end > start, `${label} source slice must be bounded`);
  return source.slice(start, end);
}

test("notary Rust runtime crates declare SDKWork component metadata", () => {
  for (const crateName of [
    "sdkwork-notary-case-contract",
    "sdkwork-notary-case-service",
    "sdkwork-notary-case-repository-sqlx",
    "sdkwork-notary-database-host",
    "sdkwork-router-notary-http-auth",
    "sdkwork-router-notary-app-api",
    "sdkwork-router-notary-backend-api",
  ]) {
    const root = path.join("crates", crateName);
    assert(existsSync(path.join(workspaceRoot, root, "README.md")), `${crateName} needs README`);
    assert(
      existsSync(path.join(workspaceRoot, root, "specs", "component.spec.json")),
      `${crateName} needs component spec`,
    );

    const spec = readJson(path.join(root, "specs", "component.spec.json"));
    assert.equal(spec.kind, "sdkwork.component.spec");
    assert.equal(spec.component.name, crateName);
    assert.equal(spec.component.domain, "notary");
    assert(spec.canonicalSpecs.some((entry) => entry.file === "RUST_CODE_SPEC.md"));
    if (crateName === "sdkwork-notary-case-repository-sqlx") {
      assert(spec.canonicalSpecs.some((entry) => entry.file === "DATABASE_SPEC.md"));
    } else if (crateName === "sdkwork-notary-database-host") {
      assert(spec.canonicalSpecs.some((entry) => entry.file === "DATABASE_FRAMEWORK_SPEC.md"));
    } else if (crateName === "sdkwork-router-notary-http-auth") {
      assert(spec.canonicalSpecs.some((entry) => entry.file === "WEB_FRAMEWORK_SPEC.md"));
    } else if (crateName.startsWith("sdkwork-router-notary-")) {
      assert(spec.canonicalSpecs.some((entry) => entry.file === "WEB_FRAMEWORK_SPEC.md"));
      assert(spec.canonicalSpecs.some((entry) => entry.file === "WEB_BACKEND_SPEC.md"));
      assert(spec.canonicalSpecs.some((entry) => entry.file === "API_SPEC.md"));
    } else {
      assert(spec.canonicalSpecs.some((entry) => entry.file === "WEB_BACKEND_SPEC.md"));
    }
  }
});

test("notary runtime service contract declares every OpenAPI operation", () => {
  const contract = notaryRuntimeContractSource();
  const operations = [
    ...openApiOperationIds("generated/openapi/notary-app-api.openapi.json"),
    ...openApiOperationIds("generated/openapi/notary-backend-api.openapi.json"),
  ];

  for (const { method, operationId } of operations) {
    assert(
      contract.includes(`"${operationId}"`),
      `notary_runtime_contract must declare ${method} ${operationId}`,
    );
  }
});

test("notary runtime dispatchers handle every OpenAPI operation", () => {
  const service = readText("crates/sdkwork-notary-case-service/src/service.rs");
  const appDispatcher = sourceSlice(
    service,
    "pub async fn handle_notary_app_operation",
    "pub async fn handle_notary_backend_operation",
    "notary app operation dispatcher",
  );
  const backendDispatcher = sourceSlice(
    service,
    "pub async fn handle_notary_backend_operation",
    "async fn list_backend_cases",
    "notary backend operation dispatcher",
  );

  for (const { method, operationId } of openApiOperationIds(
    "generated/openapi/notary-app-api.openapi.json",
  )) {
    assert(
      appDispatcher.includes(`"${operationId}"`),
      `handle_notary_app_operation must dispatch ${method} ${operationId}`,
    );
  }

  for (const { method, operationId } of openApiOperationIds(
    "generated/openapi/notary-backend-api.openapi.json",
  )) {
    assert(
      backendDispatcher.includes(`"${operationId}"`),
      `handle_notary_backend_operation must dispatch ${method} ${operationId}`,
    );
  }
});

test("notary case runtime record carries primary notary member references", () => {
  const domain = readText("crates/sdkwork-notary-case-contract/src/domain.rs");
  const service = readText("crates/sdkwork-notary-case-service/src/service.rs");
  const storage = readText("crates/sdkwork-notary-case-repository-sqlx/src/sqlite_case_repository.rs");

  const recordStart = domain.indexOf("pub struct NotaryCaseRecord");
  const recordEnd = domain.indexOf("impl NotaryCaseStatus");
  assert(recordStart > -1, "NotaryCaseRecord must exist");
  const recordSource = domain.slice(recordStart, recordEnd);

  for (const field of ["primary_notary_membership_id", "primary_notary_user_id"]) {
    assert(
      recordSource.includes(`pub ${field}: Option<String>`),
      `NotaryCaseRecord must expose ${field} for notary_case denormalized primary notary lookup`,
    );
    assert(
      service.includes(field),
      `runtime service must populate and return ${field}`,
    );
    assert(
      storage.includes(field),
      `SQLx repository must insert and load ${field}`,
    );
  }
});

test("workspace verification includes contract tests and Rust runtime tests", () => {
  const packageManifest = readJson("package.json");
  assert.equal(
    packageManifest.scripts["test:contracts"],
    "node --test sdks/test/*.test.mjs scripts/dev/*.test.mjs scripts/verify-notary-standard-architecture.test.mjs scripts/verify-notary-utils-standard.test.mjs",
  );
  assert.equal(
    packageManifest.scripts["test:topology-baggage"],
    "node --test scripts/dev/sdkwork-notary-topology-baggage.test.mjs",
  );
  assert.equal(
    packageManifest.scripts["test:rust"],
    "cargo test --workspace --target-dir target-codex-test",
  );
  assert(packageManifest.scripts.verify.includes("test:topology-validate"));
  assert(packageManifest.scripts.verify.includes("test:topology-baggage"));
  assert(packageManifest.scripts.verify.includes("db:validate"));
  assert(packageManifest.scripts.verify.includes("test:contracts"));
  assert(packageManifest.scripts.verify.includes("test:rust"));
  assert(packageManifest.scripts.verify.includes("fmt:check"));
  assert.equal(
    packageManifest.scripts["openapi:materialize"],
    "node scripts/sync-notary-openapi-authorities.mjs && node scripts/sync-notary-api-framework-metadata.mjs",
  );
  assert.equal(
    packageManifest.scripts["manifest:sync"],
    "node scripts/generate-notary-route-manifests.mjs && node scripts/sync-notary-api-framework-metadata.mjs",
  );

  const readme = readText("README.md");
  assert(readme.includes("sdkwork-notary-case-service"));
  assert(readme.includes("packages/sdkwork-im-pc-notary/src/services/NotaryService.ts"));
  assert(readme.includes("pnpm verify"));
  assert(readme.includes("pnpm test:rust"));
  assert(readme.includes("pnpm api:materialize") || readme.includes("pnpm openapi:materialize"));
  assert(readme.includes("pnpm manifest:sync"));
});

test("notary workspace uses RUST_CODE_SPEC compliant crate names", () => {
  const workspaceManifest = readText("Cargo.toml");
  for (const forbidden of [
    "sdkwork-notary-core-rust",
    "sdkwork-notary-runtime-rust",
    "sdkwork-notary-storage-sqlx-rust",
  ]) {
    assert(!workspaceManifest.includes(forbidden), `forbidden crate name must be removed: ${forbidden}`);
  }
  assert(existsSync(path.join(workspaceRoot, "specs/topology.spec.json")));
  assert(existsSync(path.join(workspaceRoot, "scripts/lib/notary-topology.mjs")));
  assert(existsSync(path.join(workspaceRoot, "apis/app-api/notary/notary-app-api.openapi.json")));
  assert(
    existsSync(path.join(workspaceRoot, "apis/backend-api/notary/notary-backend-api.openapi.json")),
  );
});
