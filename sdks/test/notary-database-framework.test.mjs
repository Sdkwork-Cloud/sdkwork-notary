import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";
import { DatabaseSync } from "node:sqlite";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { validateDatabaseFramework } from "../../../sdkwork-specs/tools/check-database-framework-standard.mjs";
import {
  buildRelativeCaseDateRefreshSql,
  resolveFixtureReferenceTime,
} from "../../database/fixtures/relative-case-dates.mjs";

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

test("notary database module uses the initialization baseline before post-GA migrations", () => {
  const manifest = readJson("database/database.manifest.json");
  assert.equal(manifest.baselineStrategy, "baseline-plus-migrations");

  for (const engine of ["postgres", "sqlite"]) {
    const baseline = `database/ddl/baseline/${engine}/0001_notary_baseline.sql`;
    assert.equal(existsSync(path.join(workspaceRoot, baseline)), true, `${baseline} required`);
    const baselineSql = readText(baseline);
    assert.match(baselineSql, /Application is in initialization state/u);
    assert.match(baselineSql, /-- sdkwork:migration/u);
    assert.match(baselineSql, /CREATE TABLE IF NOT EXISTS notary_case/u);

    const migrationDir = path.join(workspaceRoot, `database/migrations/${engine}`);
    const migrationSqlFiles = readdirSync(migrationDir)
      .filter((entry) => entry.endsWith(".sql"))
      .sort();
    assert.equal(
      migrationSqlFiles.some((entry) => entry.startsWith("0001_notary_foundation.")),
      false,
      `${engine} migrations must not duplicate the initialization baseline`,
    );
    for (const upFile of migrationSqlFiles.filter((entry) => entry.endsWith(".up.sql"))) {
      const downFile = upFile.replace(/\.up\.sql$/u, ".down.sql");
      assert(migrationSqlFiles.includes(downFile), `${engine} migration ${upFile} requires ${downFile}`);
      assert.match(readText(`database/migrations/${engine}/${upFile}`), /-- sdkwork:migration/u);
    }
  }
});

test("notary seed manifest declares locale version and empty active locale set", () => {
  const manifest = readJson("database/seeds/seed.manifest.json");
  assert.equal(manifest.i18nVersion, "1.0.0");
  assert.equal(manifest.defaultLocale, "zh-CN");
  assert.equal(manifest.fallbackLocale, "zh-CN");
  assert.deepEqual(manifest.activeLocales, ["zh-CN"]);
  assert.deepEqual(manifest.localeSets["zh-CN"].files, []);
  assert.equal(
    manifest.localeSets["zh-CN"].checksum,
    "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  );
});

test("notary integration fixtures are dev/test-only, idempotent, and owner-bounded", () => {
  const fixtureTables = [
    "notary_organization_profile",
    "notary_case",
    "notary_party",
    "notary_case_assignment",
    "notary_case_event",
  ];
  const caseStatuses = [
    "pending_review",
    "processing",
    "completed",
    "rejected",
    "cancelled",
    "create_failed",
  ];

  for (const engine of ["postgres", "sqlite"]) {
    const fixture = readText(`database/fixtures/${engine}/001_notary_integration_fixture.sql`);
    assert.match(fixture, /SDKWORK DEV\/TEST-ONLY FIXTURE/u);
    assert.match(fixture, /BEGIN(?: IMMEDIATE)?;/u);
    assert.match(fixture, /COMMIT;/u);
    assert.match(fixture, /ON CONFLICT \(id\) DO UPDATE/u);
    assert.doesNotMatch(fixture, /CREATE TABLE/u);
    assert.doesNotMatch(
      fixture,
      /\b(?:INSERT\s+INTO|UPDATE|DELETE\s+FROM)\s+(?:iam_|commerce_|dr_)/iu,
    );
    for (const table of fixtureTables) {
      assert.match(fixture, new RegExp(`INSERT INTO ${table}\\b`, "u"));
    }
    for (const status of caseStatuses) {
      assert.match(fixture, new RegExp(`'${status}'`, "u"));
    }
    assert.match(fixture, /fixture-notary-case-completed-yesterday/u);
  }

  const loader = readText("database/fixtures/load-notary-fixtures.mjs");
  assert.match(loader, /--confirm-dev-test/u);
  assert.match(loader, /SDKWORK_NOTARY_ENVIRONMENT/u);
  assert.match(loader, /allowedEnvironments/u);
  assert.match(loader, /--reference-time/u);
  assert.match(loader, /buildRelativeCaseDateRefreshSql/u);
  const fixtureReadme = readText("database/fixtures/README.md");
  assert.match(fixtureReadme, /development and automated integration tests only/u);
  assert.match(fixtureReadme, /must never read from this directory/u);
});

test("both integration fixtures can be loaded repeatedly with complete workflow coverage", () => {
  for (const engine of ["postgres", "sqlite"]) {
    const database = new DatabaseSync(":memory:");
    try {
      database.exec(readText("database/ddl/baseline/sqlite/0001_notary_baseline.sql"));
      const fixture = readText(`database/fixtures/${engine}/001_notary_integration_fixture.sql`);
      database.exec(fixture);
      database.exec(fixture);

      const count = (table) => database.prepare(`SELECT COUNT(*) AS count FROM ${table}`).get().count;
      assert.equal(count("notary_organization_profile"), 1, engine);
      assert.equal(count("notary_case"), 7, engine);
      assert.equal(count("notary_party"), 7, engine);
      assert.equal(count("notary_case_assignment"), 7, engine);
      assert.equal(count("notary_case_event"), 15, engine);
      assert.equal(
        database.prepare("SELECT COUNT(*) AS count FROM notary_case WHERE status = 'completed'").get().count,
        2,
        engine,
      );

      const statuses = database
        .prepare("SELECT DISTINCT status FROM notary_case ORDER BY status")
        .all()
        .map((row) => row.status);
      assert.deepEqual(statuses, [
        "cancelled",
        "completed",
        "create_failed",
        "pending_review",
        "processing",
        "rejected",
      ]);

      const dependencyOwnedTables = database
        .prepare(
          "SELECT name FROM sqlite_master WHERE type = 'table' AND (name LIKE 'iam_%' OR name LIKE 'commerce_%' OR name LIKE 'dr_%')",
        )
        .all();
      assert.deepEqual(dependencyOwnedTables, []);
    } finally {
      database.close();
    }
  }
});

test("relative fixture dates produce controlled today and yesterday completed workflows", () => {
  const referenceTime = resolveFixtureReferenceTime("2026-07-11T23:59:59.000Z");
  assert.throws(
    () => resolveFixtureReferenceTime("2026-07-11T23:59:59+08:00"),
    /canonical UTC ISO timestamp/u,
  );

  for (const engine of ["postgres", "sqlite"]) {
    const database = new DatabaseSync(":memory:");
    try {
      database.exec(readText("database/ddl/baseline/sqlite/0001_notary_baseline.sql"));
      database.exec(readText(`database/fixtures/${engine}/001_notary_integration_fixture.sql`));
      const refreshSql = buildRelativeCaseDateRefreshSql(engine, referenceTime);
      assert.match(refreshSql, /tenant_id = '100001'/u);
      assert.match(refreshSql, /organization_id = '200001'/u);
      assert.doesNotMatch(refreshSql, /\b(?:CREATE|INSERT|DELETE)\b/iu);
      database.exec(refreshSql);
      database.exec(refreshSql);

      const cases = database
        .prepare(`
          SELECT id, submitted_at, accepted_at, completed_at, created_at, updated_at
          FROM notary_case
          WHERE id IN ('fixture-notary-case-completed', 'fixture-notary-case-completed-yesterday')
          ORDER BY id
        `)
        .all()
        .map((row) => ({ ...row }));
      assert.deepEqual(cases, [
        {
          id: "fixture-notary-case-completed",
          submitted_at: "2026-07-11T09:00:00.000Z",
          accepted_at: "2026-07-11T10:00:00.000Z",
          completed_at: "2026-07-11T15:00:00.000Z",
          created_at: "2026-07-11T09:00:00.000Z",
          updated_at: "2026-07-11T15:00:00.000Z",
        },
        {
          id: "fixture-notary-case-completed-yesterday",
          submitted_at: "2026-07-10T09:00:00.000Z",
          accepted_at: "2026-07-10T10:00:00.000Z",
          completed_at: "2026-07-10T15:00:00.000Z",
          created_at: "2026-07-10T09:00:00.000Z",
          updated_at: "2026-07-10T15:00:00.000Z",
        },
      ], engine);

      const events = database
        .prepare(`
          SELECT id, occurred_at, created_at
          FROM notary_case_event
          WHERE case_id IN ('fixture-notary-case-completed', 'fixture-notary-case-completed-yesterday')
          ORDER BY occurred_at, id
        `)
        .all();
      assert.deepEqual(
        events.map(({ occurred_at: occurredAt, created_at: createdAt }) => [occurredAt, createdAt]),
        [
          ["2026-07-10T09:00:00.000Z", "2026-07-10T09:00:00.000Z"],
          ["2026-07-10T10:00:00.000Z", "2026-07-10T10:00:00.000Z"],
          ["2026-07-10T15:00:00.000Z", "2026-07-10T15:00:00.000Z"],
          ["2026-07-11T09:00:00.000Z", "2026-07-11T09:00:00.000Z"],
          ["2026-07-11T10:00:00.000Z", "2026-07-11T10:00:00.000Z"],
          ["2026-07-11T15:00:00.000Z", "2026-07-11T15:00:00.000Z"],
        ],
        engine,
      );

      const dashboardDates = database.prepare(`
        SELECT
          SUM(CASE WHEN date(completed_at) = date('2026-07-11') THEN 1 ELSE 0 END) AS today_count,
          SUM(CASE WHEN date(completed_at) = date('2026-07-10') THEN 1 ELSE 0 END) AS yesterday_count
        FROM notary_case
        WHERE tenant_id = '100001'
          AND organization_id = '200001'
          AND status = 'completed'
      `).get();
      assert.deepEqual({ ...dashboardDates }, { today_count: 1, yesterday_count: 1 }, engine);
    } finally {
      database.close();
    }
  }
});
