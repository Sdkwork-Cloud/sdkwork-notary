import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");

function readText(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), "utf8");
}

const routeCrates = [
  "crates/sdkwork-routes-notary-app-api",
  "crates/sdkwork-routes-notary-backend-api",
];

test("notary route crates integrate sdkwork-web-framework", () => {
  const workspaceManifest = readText("Cargo.toml");
  assert(workspaceManifest.includes("sdkwork-web-core"));
  assert(workspaceManifest.includes("sdkwork-web-axum"));
  assert(workspaceManifest.includes("sdkwork-routes-notary-http-auth"));

  for (const crateRoot of routeCrates) {
    const cargo = readText(path.join(crateRoot, "Cargo.toml"));
    assert(cargo.includes("sdkwork-web-core"));
    assert(cargo.includes("sdkwork-routes-notary-http-auth"));

    const handlers = readText(path.join(crateRoot, "src/handlers.rs"));
    const routes = readText(path.join(crateRoot, "src/routes.rs"));
    const manifest = readText(path.join(crateRoot, "src/manifest.rs"));

    assert(handlers.includes("WebRequestContext"));
    assert(handlers.includes("notary_request_context_from_web"));
    assert(routes.includes("with_dual_token_request_context"));
    assert(manifest.includes("HttpRoute::dual_token"));
    assert(manifest.includes("HttpRouteManifest"));
  }

  const httpAuth = readText("crates/sdkwork-routes-notary-http-auth/src/layer.rs");
  assert(httpAuth.includes("build_web_framework_layer"));
  assert(httpAuth.includes("with_web_request_context"));
});

test("notary route crate component specs declare web framework standard", () => {
  for (const crateRoot of [
    ...routeCrates,
    "crates/sdkwork-routes-notary-http-auth",
  ]) {
    const spec = JSON.parse(readText(path.join(crateRoot, "specs/component.spec.json")));
    assert(
      spec.canonicalSpecs.some((entry) => entry.file === "WEB_FRAMEWORK_SPEC.md"),
      `${crateRoot} must reference WEB_FRAMEWORK_SPEC.md`,
    );
  }
});

test("notary domain library does not require sdkwork-discovery without RPC services", () => {
  const workspaceSource = [
    "Cargo.toml",
    ...routeCrates.map((root) => path.join(root, "Cargo.toml")),
    "crates/sdkwork-notary-case-repository-sqlx/Cargo.toml",
  ]
    .map((relativePath) => readText(relativePath))
    .join("\n");

  assert(!workspaceSource.includes("sdkwork-discovery"));
  assert(!existsSync(path.join(workspaceRoot, "apis/rpc")));
  assert(!existsSync(path.join(workspaceRoot, "sdks/rpc")));
});
