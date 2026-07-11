# Database Fixtures

These assets are for local development and automated integration tests only. Production bootstrap,
`db:seed`, staging rollout, and deployment automation must never read from this directory.

The fixtures are deterministic and idempotent. They write only the five tables owned by
`sdkwork-notary` and deliberately do not create or modify IAM, Commerce, or Drive owner tables.
References to those domains are synthetic logical IDs. A full cross-service test environment must
provision matching dependency-owned data through the corresponding owner service or SDK.

## Coverage

| Scope | Fixture value |
| --- | --- |
| Tenant | `100001` |
| Organization | `200001` |
| Organization profile | One active notary profile |
| Cases | Seven rows covering all six storage states, including separate completed-today and completed-yesterday workflows |
| Parties | Pending, verified, failed, and expired verification states |
| Assignments | Active and released primary notary, assistant, and reviewer assignments |
| Events | Submission, acceptance, verification, completion, rejection, cancellation, and creation failure |

All names, identity material, hashes, ciphertext, order references, Drive references, and user/member
references are synthetic. They are not real credentials or personal data.

## Relative Date Coverage

The checked-in SQL remains deterministic. After applying it, the safe loader refreshes only these
owner-bounded fixture workflows using UTC dates derived from a controlled reference time:

- `fixture-notary-case-completed`: submitted at 09:00, accepted at 10:00, and completed at 15:00
  on the reference UTC date.
- `fixture-notary-case-completed-yesterday`: the same workflow timestamps on the previous UTC date.
- The three submission, acceptance, and completion events belonging to each of those cases.

The default reference time is the loader's current clock. Reproducible tests and local debugging can
set a canonical UTC ISO timestamp with `--reference-time` or
`SDKWORK_NOTARY_FIXTURE_REFERENCE_TIME`. The generated refresh SQL is restricted to tenant `100001`,
organization `200001`, and the explicit fixture case/event IDs; it does not update unrelated data.

## Load

Apply the existing engine baseline or run `pnpm db:init` before loading a fixture. The loader requires
both a development/test environment and the explicit `--confirm-dev-test` flag.

SQLite example:

```powershell
sqlite3 .runtime/notary-fixture.sqlite ".read database/ddl/baseline/sqlite/0001_notary_baseline.sql"
$env:SDKWORK_NOTARY_ENVIRONMENT = "development"
node database/fixtures/load-notary-fixtures.mjs `
  --engine sqlite `
  --database-path .runtime/notary-fixture.sqlite `
  --reference-time 2026-07-11T12:00:00.000Z `
  --confirm-dev-test
```

PostgreSQL example:

```powershell
$env:SDKWORK_NOTARY_ENVIRONMENT = "test"
$env:SDKWORK_NOTARY_DATABASE_URL = "postgres://..."
node database/fixtures/load-notary-fixtures.mjs `
  --engine postgres `
  --confirm-dev-test
```

Re-running the same command refreshes the deterministic fixture rows through `ON CONFLICT (id) DO
UPDATE`, then reapplies the relative dates for the selected reference time. It does not delete
unrelated rows.
