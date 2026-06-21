#!/usr/bin/env node

import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const generatorBin = path.resolve(repoRoot, '../sdkwork-sdk-generator/bin/sdkgen.js');

const sdkWorkspaces = [
  {
    name: 'sdkwork-notary-app-sdk',
    output: path.join(repoRoot, 'sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/generated/server-openapi'),
  },
  {
    name: 'sdkwork-notary-backend-sdk',
    output: path.join(repoRoot, 'sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/generated/server-openapi'),
  },
];

function runSdkgenInspect(outputPath) {
  return spawnSync(process.execPath, [generatorBin, 'inspect', '--output', outputPath], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
}

test('sdkgen inspect passes for notary TypeScript SDK workspaces when generator is available', () => {
  if (!existsSync(generatorBin)) {
    console.log('[sdk-generate-check] skip: sdkgen not found at', generatorBin);
    return;
  }

  for (const workspace of sdkWorkspaces) {
    assert.equal(existsSync(workspace.output), true, `${workspace.name} generated output must exist`);
    const result = runSdkgenInspect(workspace.output);
    assert.equal(
      result.status,
      0,
      `${workspace.name} sdkgen inspect failed:\n${result.stdout}\n${result.stderr}`,
    );
  }
});

test('generated TypeScript SDK entrypoints remain present', () => {
  const entrypoints = [
    path.join(repoRoot, 'sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts'),
    path.join(repoRoot, 'sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/src/index.ts'),
  ];

  for (const indexPath of entrypoints) {
    assert.equal(existsSync(indexPath), true, `${indexPath} must exist`);
    const source = readFileSync(indexPath, 'utf8');
    assert.match(source, /export/u, `${indexPath} must export generated SDK surface`);
  }
});
