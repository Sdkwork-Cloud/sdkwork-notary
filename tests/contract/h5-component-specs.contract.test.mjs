#!/usr/bin/env node

import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const packages = [
  'sdkwork-notary-h5-core',
  'sdkwork-notary-h5-commons',
  'sdkwork-notary-h5-shell',
  'sdkwork-notary-h5-notary',
];

function readJson(relativePath) {
  return JSON.parse(readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

for (const packageName of packages) {
  test(`${packageName} declares component.spec.json`, () => {
    const specPath = path.join(
      repoRoot,
      'apps/sdkwork-notary-h5/packages',
      packageName,
      'specs/component.spec.json',
    );
    assert.equal(existsSync(specPath), true, `${packageName} must declare specs/component.spec.json`);
    const spec = JSON.parse(readFileSync(specPath, 'utf8'));
    assert.equal(spec.kind, 'sdkwork.component.spec');
    assert.equal(spec.component.name, packageName);
    assert.ok(Array.isArray(spec.canonicalSpecs) && spec.canonicalSpecs.length > 0);
  });
}

test('H5 packages use workspace protocol for internal dependencies', () => {
  for (const packageName of packages) {
    const manifest = readJson(`apps/sdkwork-notary-h5/packages/${packageName}/package.json`);
    for (const [dependencyName, version] of Object.entries(manifest.dependencies ?? {})) {
      if (!dependencyName.startsWith('@sdkwork/notary-h5-')) {
        continue;
      }
      assert.equal(version, 'workspace:*', `${packageName} must use workspace:* for ${dependencyName}`);
    }
  }
});
