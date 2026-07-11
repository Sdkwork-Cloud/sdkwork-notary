#!/usr/bin/env node
/**
 * Materialize owner OpenAPI authority documents from authored apis/ inputs.
 */
import { copyFile, mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const ROOT = process.cwd();

const PAIRS = [
  {
    source: 'apis/app-api/notary/notary-app-api.openapi.json',
    target: 'generated/openapi/notary-app-api.openapi.json',
    sdkgenTarget: 'sdks/sdkwork-notary-app-sdk/openapi/notary-app-api.sdkgen.json',
  },
  {
    source: 'apis/backend-api/notary/notary-backend-api.openapi.json',
    target: 'generated/openapi/notary-backend-api.openapi.json',
    sdkgenTarget: 'sdks/sdkwork-notary-backend-sdk/openapi/notary-backend-api.sdkgen.json',
  },
];

for (const { source, target, sdkgenTarget } of PAIRS) {
  await mkdir(path.dirname(path.join(ROOT, target)), { recursive: true });
  await copyFile(path.join(ROOT, source), path.join(ROOT, target));
  console.log(`Materialized ${target} from ${source}`);

  const authority = JSON.parse(await readFile(path.join(ROOT, source), 'utf8'));
  const sdkgenDocument = materializeSdkgenDocument(authority);
  await mkdir(path.dirname(path.join(ROOT, sdkgenTarget)), { recursive: true });
  await writeFile(
    path.join(ROOT, sdkgenTarget),
    `${JSON.stringify(sdkgenDocument, null, 2)}\n`,
    'utf8',
  );
  console.log(`Materialized ${sdkgenTarget} from ${source}`);
}

function materializeSdkgenDocument(authority) {
  const sdkgenDocument = structuredClone(authority);
  const sharedResponses = sdkgenDocument.components?.responses ?? {};

  for (const pathItem of Object.values(sdkgenDocument.paths ?? {})) {
    for (const operation of Object.values(pathItem ?? {})) {
      if (!operation || typeof operation !== 'object' || !operation.responses) {
        continue;
      }
      for (const [statusCode, response] of Object.entries(operation.responses)) {
        const responseRef = response?.$ref;
        if (typeof responseRef !== 'string' || !responseRef.startsWith('#/components/responses/')) {
          continue;
        }
        const responseName = responseRef.slice('#/components/responses/'.length);
        const sharedResponse = sharedResponses[responseName];
        if (!sharedResponse) {
          throw new Error(`Missing shared response ${responseName} for HTTP ${statusCode}`);
        }
        operation.responses[statusCode] = materializeSdkgenResponse(
          sharedResponse,
          responseName,
          sdkgenDocument.components?.schemas ?? {},
        );
      }
    }
  }

  return sdkgenDocument;
}

function materializeSdkgenResponse(sharedResponse, responseName, schemas) {
  const response = structuredClone(sharedResponse);
  const schema = response.content?.['application/json']?.schema;
  const dataOverlay = schema?.allOf
    ?.map((part) => part?.properties?.data)
    .filter(Boolean)
    .at(-1);
  const dataRequired = new Set(dataOverlay?.required ?? []);
  const dataSchemaName = responseName.replace(/Response$/, '');

  if (
    dataOverlay
    && dataRequired.has('items')
    && dataRequired.has('pageInfo')
    && schemas[dataSchemaName]
  ) {
    const overlay = schema.allOf.find((part) => part?.properties?.data === dataOverlay);
    overlay.properties.data = { $ref: `#/components/schemas/${dataSchemaName}` };
  }

  return response;
}
