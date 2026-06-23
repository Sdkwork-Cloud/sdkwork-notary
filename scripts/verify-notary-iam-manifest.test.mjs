import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const HTTP_METHODS = new Set([
  'get',
  'post',
  'put',
  'patch',
  'delete',
  'head',
  'options',
  'trace',
]);

function collectOpenApiPermissions(openApiRelativePath) {
  const document = readJson(openApiRelativePath);
  const permissions = new Set();

  for (const [pathKey, pathItem] of Object.entries(document.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!HTTP_METHODS.has(method)) {
        continue;
      }
      const permission = operation?.['x-sdkwork-permission'];
      assert.ok(permission, `${openApiRelativePath} ${method.toUpperCase()} ${pathKey} must declare x-sdkwork-permission`);
      permissions.add(permission);
    }
  }

  return permissions;
}

function collectOperationPermissionMap() {
  const source = read('crates/sdkwork-notary-case-service/src/permissions.rs');
  const permissions = new Set();
  const operationPattern = /\("([^"]+)",\s*"([^"]+)"\)/g;
  for (const match of source.matchAll(operationPattern)) {
    permissions.add(match[2]);
  }
  return permissions;
}

test('IMF module manifest catalogs every OpenAPI permission authority', () => {
  const manifest = readJson('specs/iam.module.manifest.json');
  const catalogCodes = new Set(
    manifest.permissions.catalog.map((entry) => entry.code),
  );

  for (const authority of manifest.permissions.openapiAuthorities) {
    const openApiPermissions = collectOpenApiPermissions(authority);
    for (const permission of openApiPermissions) {
      assert.ok(
        catalogCodes.has(permission),
        `${permission} from ${authority} must exist in iam.module.manifest.json`,
      );
    }
  }
});

test('runtime operation permission map stays within IMF catalog', () => {
  const manifest = readJson('specs/iam.module.manifest.json');
  const catalogCodes = new Set(
    manifest.permissions.catalog.map((entry) => entry.code),
  );

  for (const permission of collectOperationPermissionMap()) {
    assert.ok(
      catalogCodes.has(permission),
      `permissions.rs maps to uncataloged permission ${permission}`,
    );
  }
});
