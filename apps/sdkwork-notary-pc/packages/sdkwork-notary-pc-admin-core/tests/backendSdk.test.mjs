import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import ts from 'typescript';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

function loadBackendSdk(stubs) {
  const filename = path.join(packageRoot, 'src/sdk/backendSdk.ts');
  const output = ts.transpileModule(readFileSync(filename, 'utf8'), {
    compilerOptions: {
      esModuleInterop: true,
      module: ts.ModuleKind.CommonJS,
      target: ts.ScriptTarget.ES2022,
    },
    fileName: filename,
  }).outputText;
  const module = { exports: {} };
  const execute = new Function('require', 'module', 'exports', output);
  execute((specifier) => stubs[specifier], module, module.exports);
  return module.exports;
}

test('admin core constructs the composed backend SDK with the PC global TokenManager', () => {
  const tokenManager = { getTokens: () => ({ accessToken: 'token' }) };
  const configs = [];
  const client = { notary: {} };
  const backendSdk = loadBackendSdk({
    '@sdkwork/notary-backend-sdk': {
      createClient(config) {
        configs.push(config);
        return client;
      },
    },
    '@sdkwork/notary-pc-core': {
      getNotaryPcGlobalTokenManager: () => tokenManager,
    },
    '@sdkwork/utils': {
      trim: (value) => value.trim(),
    },
  });

  assert.equal(
    backendSdk.createNotaryPcAdminBackendSdkClient({ baseUrl: ' https://api.example.test/ ' }),
    client,
  );
  assert.deepEqual(configs[0], {
    baseUrl: 'https://api.example.test',
    platform: 'pc',
    tokenManager,
  });
  assert.equal(backendSdk.initNotaryPcAdminBackendSdkClient({ baseUrl: '/gateway/' }), client);
  assert.equal(backendSdk.getNotaryPcAdminBackendSdkClient(), client);
  backendSdk.resetNotaryPcAdminBackendSdkClient();
  assert.throws(() => backendSdk.getNotaryPcAdminBackendSdkClient(), /not initialized/u);
  assert.throws(
    () => backendSdk.createNotaryPcAdminBackendSdkClient({ baseUrl: '   ' }),
    /base URL is required/u,
  );
});

test('admin core remains the only backend SDK boundary and declares backend-admin metadata', () => {
  const source = readFileSync(path.join(packageRoot, 'src/sdk/backendSdk.ts'), 'utf8');
  const manifest = JSON.parse(readFileSync(path.join(packageRoot, 'package.json'), 'utf8'));
  const component = JSON.parse(
    readFileSync(path.join(packageRoot, 'specs/component.spec.json'), 'utf8'),
  );

  assert.match(source, /@sdkwork\/notary-backend-sdk/u);
  assert.match(source, /getNotaryPcGlobalTokenManager/u);
  for (const forbidden of ['fetch(', 'axios', 'Authorization', 'Access-Token', '/app/v3/api']) {
    assert.equal(source.includes(forbidden), false, `source must not contain ${forbidden}`);
  }
  assert.equal(manifest.sdkwork.architecture, 'pc-admin');
  assert.equal(manifest.sdkwork.surface, 'backend-admin');
  assert.equal(component.component.type, 'react-package');
  assert.equal(component.component.capability, 'admin-core');
});
