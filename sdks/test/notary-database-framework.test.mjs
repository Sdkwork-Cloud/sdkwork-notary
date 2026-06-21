import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { validateDatabaseFramework } from "../../../sdkwork-specs/tools/check-database-framework-standard.mjs";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");

function readJson(relativePath) {
  return JSON.parse(readFileSync(path.join(workspaceRoot, relativePath), "utf8"));
}

function readText(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), "utf8");
}

test("notary database module satisfies DATABASE_FRAMEWORK_SPEC", () => {
  const result = validateDatabaseFramework(workspaceRoot);
  assert.equal(result.skipped, false, "notary must own database/");
  assert.equal(result.ok, true, `database framework validation failed: ${result.failures.join("; ")}`);
});

test("notary verify aggregate includes database framework validation", () => {
  const packageManifest = readJson("package.json");
  assert(packageManifest.scripts.verify.includes("check"));
  assert(packageManifest.scripts.check.includes("db:validate"));
  assert.equal(packageManifest.scripts["db:validate"], packageManifest.scripts["test:contract:database"]);
});

test("notary database manifest declares module ownership", () => {
  assert.equal(existsSync(path.join(workspaceRoot, "database/database.manifest.json")), true);
  const manifest = readJson("database/database.manifest.json");
  assert.equal(manifest.moduleId, "notary");
  assert(manifest.engines.includes("postgres"));
  assert(manifest.engines.includes("sqlite"));
});

test("notary database module declares versioned migrations", () => {
  for (const engine of ["postgres", "sqlite"]) {
    const up = `database/migrations/${engine}/0001_notary_foundation.up.sql`;
    const down = `database/migrations/${engine}/0001_notary_foundation.down.sql`;
    assert.equal(existsSync(path.join(workspaceRoot, up)), true, `${up} required`);
    assert.equal(existsSync(path.join(workspaceRoot, down)), true, `${down} required`);
    const upSql = readText(up);
    assert.match(upSql, /-- sdkwork:migration/u);
    assert.match(upSql, /CREATE TABLE IF NOT EXISTS notary_case/u);
  }
});
