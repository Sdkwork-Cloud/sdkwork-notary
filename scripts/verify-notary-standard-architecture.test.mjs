import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const ROOT = process.cwd();

const STANDARD_ROOT_DIRECTORIES = [
  'apis',
  'apps',
  'crates',
  'sdks',
  'jobs',
  'tools',
  'plugins',
  'examples',
  'configs',
  'deployments',
  'scripts',
  'docs',
  'tests',
];

const REQUIRED_WORKSPACE_FILES = [
  'AGENTS.md',
  'CLAUDE.md',
  'CODEX.md',
  'GEMINI.md',
  'README.md',
  'Cargo.toml',
  'sdkwork.workflow.json',
  '.github/workflows/package.yml',
  '.sdkwork/README.md',
  '.sdkwork/.gitignore',
  '.sdkwork/skills/README.md',
  '.sdkwork/plugins/README.md',
  'docs/root-layout.md',
];

const API_INPUTS = [
  'apis/app-api/notary/notary-app-api.openapi.json',
  'apis/backend-api/notary/notary-backend-api.openapi.json',
];

const ROUTE_MANIFESTS = [
  {
    file: 'sdks/_route-manifests/app-api/sdkwork-router-notary-app-api.route-manifest.json',
    apiSurface: 'app-api',
  },
  {
    file: 'sdks/_route-manifests/backend-api/sdkwork-router-notary-backend-api.route-manifest.json',
    apiSurface: 'backend-api',
  },
];

const WEB_FRAMEWORK_CRATES = [
  'crates/sdkwork-router-notary-app-api/Cargo.toml',
  'crates/sdkwork-router-notary-backend-api/Cargo.toml',
  'crates/sdkwork-router-notary-http-auth/Cargo.toml',
];

function read(relativePath) {
  return readFileSync(path.join(ROOT, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function exists(relativePath) {
  return existsSync(path.join(ROOT, relativePath));
}

test('declares SDKWork standard root directory dictionary', () => {
  for (const directory of STANDARD_ROOT_DIRECTORIES) {
    assert.equal(exists(directory), true, `${directory}/ should exist`);
    assert.equal(exists(path.join(directory, 'README.md')), true, `${directory}/README.md should exist`);
  }
});

test('declares workspace agent entrypoints and packaging workflow', () => {
  for (const file of REQUIRED_WORKSPACE_FILES) {
    assert.equal(exists(file), true, `${file} should exist`);
  }

  const workflow = readJson('sdkwork.workflow.json');
  assert.equal(workflow.app.id, 'sdkwork-notary');
  assert.equal(workflow.app.configPath, 'sdkwork.app.config.json');
});

test('declares author-owned API contracts under apis/', () => {
  for (const file of API_INPUTS) {
    assert.equal(exists(file), true, `${file} should exist`);
    const openapi = readJson(file);
    assert.equal(openapi.openapi, '3.1.2');
  }
});

test('integrates sdkwork-web-framework in HTTP route crates', () => {
  const rootCargo = read('Cargo.toml');
  assert.match(rootCargo, /sdkwork-web-core/);
  assert.match(rootCargo, /sdkwork-web-axum/);
  assert.match(rootCargo, /sdkwork-iam-web-adapter/);

  for (const cargoPath of WEB_FRAMEWORK_CRATES) {
    const cargo = read(cargoPath);
    assert.match(cargo, /sdkwork-web-/);
  }

  const httpAuthLayer = read('crates/sdkwork-router-notary-http-auth/src/layer.rs');
  assert.match(httpAuthLayer, /build_web_framework_layer/);
  assert.match(httpAuthLayer, /with_web_request_context/);
});

test('integrates sdkwork-database in notary SQLx repository crate', () => {
  const repositoryCargo = read('crates/sdkwork-notary-case-repository-sqlx/Cargo.toml');
  assert.match(repositoryCargo, /sdkwork-database-config/);
  assert.match(repositoryCargo, /sdkwork-database-sqlx/);
});

test('does not declare sdkwork-discovery without RPC services', () => {
  const rootCargo = read('Cargo.toml');
  assert.doesNotMatch(rootCargo, /sdkwork-discovery/);
  assert.equal(exists('apis/rpc'), false, 'RPC contracts should not exist yet');
});

test('route manifests declare WebRequestContext and apiSurface on every route', () => {
  for (const { file, apiSurface } of ROUTE_MANIFESTS) {
    const manifest = readJson(file);
    assert.ok(Array.isArray(manifest.routes) && manifest.routes.length > 0, `${file} should declare routes`);
    for (const route of manifest.routes) {
      assert.equal(route.requestContext, 'WebRequestContext', `${route.method} ${route.path} missing requestContext`);
      assert.equal(route.apiSurface, apiSurface, `${route.method} ${route.path} missing apiSurface`);
      assert.ok(route.handler?.name, `${route.operationId} missing handler.name`);
      assert.ok(route.auth?.mode, `${route.operationId} missing auth.mode`);
    }
  }
});

test('OpenAPI authorities declare x-sdkwork-request-context and x-sdkwork-api-surface', () => {
  const expectations = [
    { file: 'apis/app-api/notary/notary-app-api.openapi.json', apiSurface: 'app-api' },
    { file: 'apis/backend-api/notary/notary-backend-api.openapi.json', apiSurface: 'backend-api' },
    { file: 'generated/openapi/notary-app-api.openapi.json', apiSurface: 'app-api' },
    { file: 'generated/openapi/notary-backend-api.openapi.json', apiSurface: 'backend-api' },
  ];

  const methods = new Set(['get', 'post', 'put', 'patch', 'delete']);

  for (const { file, apiSurface } of expectations) {
    const openapi = readJson(file);
    let operationCount = 0;
    for (const pathItem of Object.values(openapi.paths ?? {})) {
      for (const [method, operation] of Object.entries(pathItem ?? {})) {
        if (!methods.has(method)) {
          continue;
        }
        operationCount += 1;
        assert.equal(
          operation['x-sdkwork-request-context'],
          'WebRequestContext',
          `${file} ${method} missing x-sdkwork-request-context`,
        );
        assert.equal(
          operation['x-sdkwork-api-surface'],
          apiSurface,
          `${file} ${method} missing x-sdkwork-api-surface`,
        );
        assert.equal(
          operation['x-sdkwork-auth-mode'],
          'dual-token',
          `${file} ${operation.operationId ?? method} missing x-sdkwork-auth-mode`,
        );
        assert.ok(
          operation['x-sdkwork-source-route-crate'],
          `${file} ${operation.operationId ?? method} missing x-sdkwork-source-route-crate`,
        );
        assert.ok(
          operation['x-sdkwork-source'],
          `${file} ${operation.operationId ?? method} missing x-sdkwork-source`,
        );
      }
    }
    assert.ok(operationCount > 0, `${file} should contain HTTP operations`);
  }
});

test('declares topology profiles for host application wiring', () => {
  assert.equal(exists('specs/topology.spec.json'), true);

  const topology = readJson('specs/topology.spec.json');
  for (const profileFile of Object.values(topology.profileFiles ?? {})) {
    assert.equal(exists(profileFile), true, `${profileFile} should exist`);
  }

  assert.equal(topology.database?.appPrefix, 'SDKWORK_NOTARY');
  assert.deepEqual(topology.surfaces?.['application.public-ingress']?.protocols, ['http']);
});

test('domain library root declares sdkwork.app.config.json', () => {
  assert.equal(exists('sdkwork.app.config.json'), true);

  const manifest = readJson('sdkwork.app.config.json');
  assert.equal(manifest.schemaVersion, 3);
  assert.equal(manifest.kind, 'sdkwork.app');
  assert.equal(manifest.app.key, 'sdkwork-notary');
});

test('Rust HTTP crates follow sdkwork-router-notary-* naming', () => {
  const expectedMembers = [
    'crates/sdkwork-router-notary-app-api',
    'crates/sdkwork-router-notary-backend-api',
    'crates/sdkwork-router-notary-http-auth',
    'crates/sdkwork-notary-case-contract',
    'crates/sdkwork-notary-case-service',
    'crates/sdkwork-notary-case-repository-sqlx',
  ];

  const cargo = read('Cargo.toml');
  for (const member of expectedMembers) {
    assert.match(cargo, new RegExp(`"${member.replaceAll('/', '\\/')}"`));
  }
});

test('route crate component specs reference canonical route manifest files', () => {
  assert.equal(
    readJson('crates/sdkwork-router-notary-app-api/specs/component.spec.json').contracts.routeManifest,
    '../../../sdks/_route-manifests/app-api/sdkwork-router-notary-app-api.route-manifest.json',
  );
  assert.equal(
    readJson('crates/sdkwork-router-notary-backend-api/specs/component.spec.json').contracts.routeManifest,
    '../../../sdks/_route-manifests/backend-api/sdkwork-router-notary-backend-api.route-manifest.json',
  );
});

test('declares contract maintenance scripts and schema registry', () => {
  const packageManifest = readJson('package.json');
  assert.equal(
    packageManifest.scripts['openapi:materialize'],
    'node scripts/sync-notary-openapi-authorities.mjs && node scripts/sync-notary-api-framework-metadata.mjs',
  );
  assert.equal(
    packageManifest.scripts['manifest:sync'],
    'node scripts/generate-notary-route-manifests.mjs && node scripts/sync-notary-api-framework-metadata.mjs',
  );

  assert.equal(exists('specs/README.md'), true);
  assert.equal(exists('generated/README.md'), true);
  assert.equal(exists('generated/openapi/README.md'), true);
  assert.equal(exists('docs/schema-registry/README.md'), true);
  assert.equal(exists('docs/schema-registry/tables/001-notary-core.yaml'), true);
});
