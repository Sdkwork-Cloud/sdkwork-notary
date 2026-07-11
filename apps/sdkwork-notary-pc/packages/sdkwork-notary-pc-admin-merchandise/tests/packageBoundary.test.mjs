import assert from 'node:assert/strict';
import { readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import { packageRoot } from './typescriptModuleLoader.mjs';

function listFiles(directory) {
  return readdirSync(directory).flatMap((entry) => {
    const fullPath = path.join(directory, entry);
    return statSync(fullPath).isDirectory() ? listFiles(fullPath) : [fullPath];
  });
}

test('admin merchandise package stays on the injected backend SDK boundary', () => {
  const source = listFiles(path.join(packageRoot, 'src'))
    .filter((file) => /\.(?:ts|tsx)$/u.test(file))
    .map((file) => readFileSync(file, 'utf8'))
    .join('\n');

  for (const forbidden of [
    'fetch(',
    'axios',
    'Authorization',
    'Access-Token',
    'generated/server-openapi',
    '@sdkwork/order-',
    '@sdkwork/payment-',
    '@sdkwork/commerce-',
  ]) {
    assert.equal(source.includes(forbidden), false, `source must not contain ${forbidden}`);
  }

  assert.match(source, /backendClient\.notary\.matters\.management\.list/u);
  assert.match(source, /backendClient\.notary\.matters\.create/u);
  assert.match(source, /backendClient\.notary\.matters\.update/u);
  assert.match(source, /status: filters\.status/u);
  assert.equal(source.includes('.slice('), false);
});

test('component metadata and i18n fragments declare the backend-admin merchandise surface', () => {
  const manifest = JSON.parse(readFileSync(path.join(packageRoot, 'package.json'), 'utf8'));
  const component = JSON.parse(
    readFileSync(path.join(packageRoot, 'specs/component.spec.json'), 'utf8'),
  );

  assert.equal(manifest.sdkwork.architecture, 'pc-admin');
  assert.equal(manifest.sdkwork.surface, 'backend-admin');
  assert.equal(manifest.sdkwork.capability, 'merchandise');
  assert.equal(component.component.type, 'react-package');
  assert.equal(component.component.surface, 'backend-admin');
  assert.equal(component.component.capability, 'merchandise');
  for (const locale of ['en-US', 'zh-CN']) {
    assert.equal(
      statSync(path.join(packageRoot, `src/i18n/${locale}/notary/merchandise/matters.json`)).isFile(),
      true,
    );
  }
});
