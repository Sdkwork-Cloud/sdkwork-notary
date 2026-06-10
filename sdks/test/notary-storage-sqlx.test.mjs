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

function readJson(relativePath) {
  return JSON.parse(readText(relativePath));
}

test("notary SQLx storage crate is part of the workspace and verify command", () => {
  const workspaceManifest = readText("Cargo.toml");
  assert(workspaceManifest.includes('"crates/sdkwork-notary-storage-sqlx-rust"'));

  const packageManifest = readJson("package.json");
  assert.equal(
    packageManifest.scripts["test:rust"],
    "cargo test --workspace --target-dir target-codex-test",
  );
  assert(packageManifest.scripts.verify.includes("test:rust"));

  const crateRoot = "crates/sdkwork-notary-storage-sqlx-rust";
  assert(existsSync(path.join(workspaceRoot, crateRoot, "Cargo.toml")));
  assert(existsSync(path.join(workspaceRoot, crateRoot, "src", "lib.rs")));
  assert(existsSync(path.join(workspaceRoot, crateRoot, "src", "sqlite_case_repository.rs")));
  assert(existsSync(path.join(workspaceRoot, crateRoot, "tests", "sqlite_case_repository.rs")));
});

test("notary SQLx storage keeps ownership local and does not join dependency tables", () => {
  const source = readText(
    "crates/sdkwork-notary-storage-sqlx-rust/src/sqlite_case_repository.rs",
  );

  for (const api of [
    "SqliteNotaryCaseRepository",
    "notary_foundation_migration_sql",
    "upsert_organization_profile",
    "list_organization_profiles",
    "insert_case",
    "insert_party",
    "append_event",
    "get_case",
    "list_parties",
    "list_events",
  ]) {
    assert(source.includes(api), `${api} must be implemented`);
  }

  for (const forbidden of [
    "JOIN commerce_",
    "JOIN dr_drive_",
    "JOIN iam_",
    "FROM commerce_",
    "FROM dr_drive_",
    "FROM iam_",
  ]) {
    assert(!source.includes(forbidden), `storage must not query dependency table: ${forbidden}`);
  }

  assert(source.includes("drive_space_type"));
  assert(source.includes("drive_folder_node_id"));
  assert(source.includes("sku_id"));
  assert(source.includes("order_item_id"));
});
