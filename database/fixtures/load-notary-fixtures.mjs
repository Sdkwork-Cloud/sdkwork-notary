#!/usr/bin/env node

import { readFileSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import {
  buildRelativeCaseDateRefreshSql,
  resolveFixtureReferenceTime,
} from './relative-case-dates.mjs';

const fixtureRoot = path.dirname(fileURLToPath(import.meta.url));
const allowedEnvironments = new Set(['dev', 'development', 'local', 'test']);

function fail(message) {
  process.stderr.write(`${message}\n`);
  process.exit(1);
}

function printHelp() {
  process.stdout.write(`Usage:
  node database/fixtures/load-notary-fixtures.mjs --engine sqlite --database-path <path> --confirm-dev-test
  node database/fixtures/load-notary-fixtures.mjs --engine postgres --confirm-dev-test

Optional deterministic clock:
  --reference-time 2026-07-11T12:00:00.000Z

Required environment:
  SDKWORK_NOTARY_ENVIRONMENT=dev|development|local|test
  SDKWORK_NOTARY_DATABASE_URL=<postgres URL>  # postgres only
  SDKWORK_NOTARY_FIXTURE_REFERENCE_TIME=<canonical UTC ISO timestamp>  # optional
`);
}

function parseArgs(argv) {
  const options = {
    confirmDevTest: false,
    databasePath: undefined,
    engine: undefined,
    help: false,
    referenceTime: undefined,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const token = argv[index];
    if (token === '--confirm-dev-test') {
      options.confirmDevTest = true;
    } else if (token === '--database-path') {
      options.databasePath = requireArgumentValue(argv, index, token);
      index += 1;
    } else if (token === '--engine') {
      options.engine = requireArgumentValue(argv, index, token);
      index += 1;
    } else if (token === '--reference-time') {
      options.referenceTime = requireArgumentValue(argv, index, token);
      index += 1;
    } else if (token === '--help' || token === '-h') {
      options.help = true;
    } else {
      fail(`Unknown fixture loader argument: ${token}`);
    }
  }

  return options;
}

function requireArgumentValue(argv, index, optionName) {
  const value = argv[index + 1];
  if (!value || value.startsWith('--')) {
    fail(`${optionName} requires a value.`);
  }
  return value;
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    encoding: 'utf8',
    ...options,
  });

  if (result.stdout) {
    process.stdout.write(result.stdout);
  }
  if (result.stderr) {
    process.stderr.write(result.stderr);
  }
  if (result.error) {
    fail(`Unable to execute ${command}: ${result.error.message}`);
  }
  if (result.status !== 0) {
    fail(`${command} exited with status ${result.status}`);
  }
}

const options = parseArgs(process.argv.slice(2));
if (options.help) {
  printHelp();
  process.exit(0);
}

const environment = (process.env.SDKWORK_NOTARY_ENVIRONMENT ?? process.env.NODE_ENV ?? '')
  .trim()
  .toLowerCase();

if (!options.confirmDevTest) {
  fail('Refusing to load fixtures without --confirm-dev-test.');
}
if (!allowedEnvironments.has(environment)) {
  fail('Refusing to load fixtures unless SDKWORK_NOTARY_ENVIRONMENT is dev, development, local, or test.');
}
if (options.engine !== 'sqlite' && options.engine !== 'postgres') {
  fail('--engine must be sqlite or postgres.');
}

const fixturePath = path.join(fixtureRoot, options.engine, '001_notary_integration_fixture.sql');
let referenceTime;
try {
  referenceTime = resolveFixtureReferenceTime(
    options.referenceTime ?? process.env.SDKWORK_NOTARY_FIXTURE_REFERENCE_TIME,
  );
} catch (error) {
  fail(error instanceof Error ? error.message : String(error));
}
const fixtureSql = readFileSync(fixturePath, 'utf8');
const relativeDateSql = buildRelativeCaseDateRefreshSql(options.engine, referenceTime);
const loadSql = `${fixtureSql.trimEnd()}\n\n${relativeDateSql}`;

if (options.engine === 'sqlite') {
  if (!options.databasePath?.trim()) {
    fail('--database-path is required for SQLite fixtures.');
  }
  const databasePath = options.databasePath === ':memory:'
    ? ':memory:'
    : path.resolve(options.databasePath);
  run('sqlite3', [databasePath], { input: loadSql });
} else {
  const databaseUrl = process.env.SDKWORK_NOTARY_DATABASE_URL?.trim();
  if (!databaseUrl) {
    fail('SDKWORK_NOTARY_DATABASE_URL is required for PostgreSQL fixtures.');
  }
  run('psql', ['--set', 'ON_ERROR_STOP=on'], {
    env: {
      ...process.env,
      PGDATABASE: databaseUrl,
    },
    input: loadSql,
  });
}

process.stdout.write(
  `Loaded ${options.engine} notary dev/test fixture with reference time ${referenceTime.toISOString()}.\n`,
);
