#!/usr/bin/env node
/**
 * Materialize owner OpenAPI authority documents from authored apis/ inputs.
 */
import { copyFile, mkdir } from 'node:fs/promises';
import path from 'node:path';

const ROOT = process.cwd();

const PAIRS = [
  {
    source: 'apis/app-api/notary/notary-app-api.openapi.json',
    target: 'generated/openapi/notary-app-api.openapi.json',
  },
  {
    source: 'apis/backend-api/notary/notary-backend-api.openapi.json',
    target: 'generated/openapi/notary-backend-api.openapi.json',
  },
];

for (const { source, target } of PAIRS) {
  await mkdir(path.dirname(path.join(ROOT, target)), { recursive: true });
  await copyFile(path.join(ROOT, source), path.join(ROOT, target));
  console.log(`Materialized ${target} from ${source}`);
}
