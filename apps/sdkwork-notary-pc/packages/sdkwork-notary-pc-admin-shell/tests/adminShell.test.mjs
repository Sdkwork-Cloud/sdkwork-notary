import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const read = (relativePath) => readFileSync(path.join(packageRoot, relativePath), 'utf8');

test('admin shell owns the /admin route, permission guard, navigation, and lazy capability loading', () => {
  const routes = read('src/routes/AdminRoutes.tsx');
  const manifest = read('src/routes/adminRouteManifest.ts');
  const layout = read('src/components/AdminShellLayout.tsx');

  assert.match(manifest, /NOTARY_PC_ADMIN_ROOT_PATH = '\/admin'/u);
  assert.match(manifest, /NOTARY_PC_ADMIN_MATTERS_PATH = '\/admin\/notary\/matters'/u);
  assert.match(routes, /NotaryPcAdminPermissionGate/u);
  assert.match(routes, /import\('@sdkwork\/notary-pc-admin-merchandise'\)/u);
  assert.match(routes, /lazy\(/u);
  assert.match(layout, /hasNotaryPcAdminPermission/u);
  assert.match(layout, /NavLink/u);
});

test('admin shell contains no business transport and declares backend-admin metadata', () => {
  const source = [
    read('src/routes/AdminRoutes.tsx'),
    read('src/routes/adminRouteManifest.ts'),
    read('src/components/AdminShellLayout.tsx'),
  ].join('\n');
  const packageManifest = JSON.parse(read('package.json'));
  const component = JSON.parse(read('specs/component.spec.json'));

  for (const forbidden of [
    'fetch(',
    'axios',
    '@sdkwork/notary-backend-sdk',
    'Authorization',
    'Access-Token',
  ]) {
    assert.equal(source.includes(forbidden), false, `source must not contain ${forbidden}`);
  }
  assert.equal(packageManifest.sdkwork.architecture, 'pc-admin');
  assert.equal(packageManifest.sdkwork.surface, 'backend-admin');
  assert.equal(component.component.type, 'react-package');
  assert.equal(component.component.capability, 'admin-shell');
});
