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

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function listPackageNames() {
  const packagesDir = path.join(appRoot, 'packages');
  return readdirSync(packagesDir).filter((entry) => statSync(path.join(packagesDir, entry)).isDirectory());
}

const REQUIRED_LAYOUT_PATHS = [
  'AGENTS.md',
  'sdkwork.app.config.json',
  '.sdkwork/README.md',
  '.sdkwork/skills/README.md',
  '.sdkwork/plugins/README.md',
  'bin/windows/README.md',
  'bin/linux/README.md',
  'bin/macos/README.md',
  'config/browser/runtime-env.development.example.json',
  'config/browser/runtime-env.test.example.json',
  'config/browser/runtime-env.staging.example.json',
  'config/browser/runtime-env.production.example.json',
  'config/desktop/notary.development.toml.example',
  'config/desktop/notary.test.toml.example',
  'config/desktop/notary.staging.toml.example',
  'config/desktop/notary.production.toml.example',
  'config/server/notary.development.toml.example',
  'config/server/notary.test.toml.example',
  'config/server/notary.staging.toml.example',
  'config/server/notary.production.toml.example',
  'config/container/notary.development.toml.example',
  'config/container/notary.test.toml.example',
  'config/container/notary.staging.toml.example',
  'config/container/notary.production.toml.example',
  'docs/README.md',
  'public/README.md',
  'scripts/README.md',
  'sdks/README.md',
  'specs/README.md',
  'tests/README.md',
  'src/main.tsx',
  'src/App.tsx',
  'src/AuthGate.tsx',
  'src/providers/README.md',
  'src/shell/README.md',
  'src/routes/README.md',
  'src/bootstrap/environment.ts',
  'src/bootstrap/tokenManager.ts',
  'src/bootstrap/runtime.ts',
  'src/bootstrap/sdkClients.ts',
  'src/bootstrap/iamRuntime.ts',
  'src/bootstrap/routes.ts',
  'packages/sdkwork-notary-pc-core/package.json',
  'packages/sdkwork-notary-pc-commons/package.json',
  'packages/sdkwork-notary-pc-shell/package.json',
  'packages/sdkwork-notary-pc-notary/package.json',
  'packages/sdkwork-notary-pc-core/specs/component.spec.json',
  'packages/sdkwork-notary-pc-commons/specs/component.spec.json',
  'packages/sdkwork-notary-pc-shell/specs/component.spec.json',
  'packages/sdkwork-notary-pc-notary/specs/component.spec.json',
];

test('notary pc root follows APP_PC_ARCHITECTURE_SPEC layout', () => {
  for (const relativePath of REQUIRED_LAYOUT_PATHS) {
    assert.equal(existsSync(path.join(appRoot, relativePath)), true, `missing ${relativePath}`);
  }

  const manifest = readJson('sdkwork.app.config.json');
  assert.equal(manifest.schemaVersion, 3);
  assert.equal(manifest.kind, 'sdkwork.app');
  assert.equal(manifest.app.key, 'sdkwork-notary-pc');
  assert.equal(manifest.runtime.family, 'pc');
  assert.equal(manifest.runtime.framework, 'react-pc');

  for (const required of [
    'sdkwork-notary-pc-core',
    'sdkwork-notary-pc-commons',
    'sdkwork-notary-pc-shell',
    'sdkwork-notary-pc-notary',
  ]) {
    assert(listPackageNames().includes(required), `packages must include ${required}`);
  }
});

test('notary pc root keeps thin bootstrap and host-port based notary package', () => {
  const app = read('src/App.tsx');
  const runtime = read('src/bootstrap/runtime.ts');
  const sdkClients = read('src/bootstrap/sdkClients.ts');
  const environment = read('src/bootstrap/environment.ts');
  const notaryService = read('packages/sdkwork-notary-pc-notary/src/services/NotaryService.ts');
  const notaryI18n = read('packages/sdkwork-notary-pc-notary/src/i18n/index.ts');
  const host = read('packages/sdkwork-notary-pc-commons/src/host/notaryPcHost.ts');
  const core = read('packages/sdkwork-notary-pc-core/src/index.ts');

  assert(app.includes('AuthGate'));
  assert(app.includes('bootstrap()'));
  assert(runtime.includes('bootstrapSdkClients'));
  assert(sdkClients.includes('configureNotaryPcRuntime'));
  assert(sdkClients.includes('createDefaultBrowserHostAdapter'));
  assert(environment.includes('resolveEnvironment'));
  assert(core.includes('configureNotaryPcSdkPorts'));
  assert(host.includes('configureNotaryPcHost'));
  assert(notaryService.includes('createNotaryPcService'));
  assert(notaryService.includes('getConfiguredNotaryAppSdkClient'));
  assert(notaryI18n.includes('getNotaryPcHost().onLanguageChange'));
  assert(notaryI18n.includes('resolveInitialLanguage'));

  for (const forbidden of ['fetch(', 'axios', 'Authorization', 'Access-Token', 'picsum.photos']) {
    assert(!notaryService.includes(forbidden), `NotaryService must not include ${forbidden}`);
  }
});
