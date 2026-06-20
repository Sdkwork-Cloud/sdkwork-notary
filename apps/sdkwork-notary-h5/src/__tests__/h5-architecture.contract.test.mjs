#!/usr/bin/env node

import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function listPackageNames() {
  const packagesDir = path.join(appRoot, 'packages');
  return readdirSync(packagesDir).filter((entry) => statSync(path.join(packagesDir, entry)).isDirectory());
}

test('notary h5 root follows APP_H5_ARCHITECTURE_SPEC layout', () => {
  for (const relativePath of [
    'AGENTS.md',
    'sdkwork.app.config.json',
    '.sdkwork/README.md',
    'config/browser/runtime-env.development.example.json',
    'config/host/capacitor.development.example.json',
    'src/main.tsx',
    'src/App.tsx',
    'src/AuthGate.tsx',
    'src/bootstrap/runtime.ts',
    'src/bootstrap/sdkClients.ts',
    'src/bootstrap/iamRuntime.ts',
    'src/bootstrap/hostAdapters.ts',
    'src/bootstrap/routes.ts',
    'packages/sdkwork-notary-h5-core/package.json',
    'packages/sdkwork-notary-h5-commons/package.json',
    'packages/sdkwork-notary-h5-shell/package.json',
    'packages/sdkwork-notary-h5-notary/package.json',
  ]) {
    assert.equal(existsSync(path.join(appRoot, relativePath)), true, `missing ${relativePath}`);
  }

  const packageNames = listPackageNames();
  for (const required of [
    'sdkwork-notary-h5-core',
    'sdkwork-notary-h5-commons',
    'sdkwork-notary-h5-shell',
    'sdkwork-notary-h5-notary',
  ]) {
    assert(packageNames.includes(required), `packages must include ${required}`);
  }
});

test('notary h5 root keeps a thin bootstrap shell and SDK-backed notary package', () => {
  const app = read('src/App.tsx');
  const runtime = read('src/bootstrap/runtime.ts');
  const sdkClients = read('src/bootstrap/sdkClients.ts');
  const notaryHome = read('packages/sdkwork-notary-h5-notary/src/pages/NotaryHomePage.tsx');
  const notaryService = read('packages/sdkwork-notary-h5-notary/src/services/notaryH5Service.ts');
  const core = read('packages/sdkwork-notary-h5-core/src/index.ts');

  assert(app.includes('BrowserRouter'));
  assert(app.includes('AuthGate'));
  assert(app.includes('NotaryApp'));
  assert(app.includes('bootstrap()'));
  assert(runtime.includes('createIamRuntime'));
  assert(runtime.includes('bootstrapSdkClients'));
  assert(sdkClients.includes('initNotaryH5AppSdkClient'));
  assert(sdkClients.includes("platform: 'h5'"));
  assert(core.includes('createNotaryApi'));
  assert(notaryService.includes('getNotaryH5ComposedApi'));
  assert(notaryService.includes('getDashboardStatistics'));
  assert(notaryService.includes('listCases'));
  assert(notaryHome.includes('createNotaryH5Service'));
  assert(notaryHome.includes('Recent cases'));

  for (const forbidden of ['fetch(', 'axios', 'Authorization', 'Access-Token', 'picsum.photos']) {
    assert(!notaryService.includes(forbidden), `notaryH5Service must not include ${forbidden}`);
    assert(!notaryHome.includes(forbidden), `NotaryHomePage must not include ${forbidden}`);
  }
});
