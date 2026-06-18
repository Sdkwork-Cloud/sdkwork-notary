#!/usr/bin/env node
/**
 * Sync SDKWork web-framework and route-manifest contract metadata into Notary
 * route manifests and OpenAPI authorities.
 */
import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const ROOT = process.cwd();

const ROUTE_MANIFESTS = [
  {
    file: 'sdks/_route-manifests/app-api/sdkwork-router-notary-app-api.route-manifest.json',
    apiSurface: 'app-api',
  },
  {
    file: 'sdks/_route-manifests/backend-api/sdkwork-router-notary-backend-api.route-manifest.json',
    apiSurface: 'backend-api',
  },
];

const OPENAPI_FILES = [
  {
    file: 'apis/app-api/notary/notary-app-api.openapi.json',
    apiSurface: 'app-api',
    manifestFile:
      'sdks/_route-manifests/app-api/sdkwork-router-notary-app-api.route-manifest.json',
  },
  {
    file: 'apis/backend-api/notary/notary-backend-api.openapi.json',
    apiSurface: 'backend-api',
    manifestFile:
      'sdks/_route-manifests/backend-api/sdkwork-router-notary-backend-api.route-manifest.json',
  },
  {
    file: 'generated/openapi/notary-app-api.openapi.json',
    apiSurface: 'app-api',
    manifestFile:
      'sdks/_route-manifests/app-api/sdkwork-router-notary-app-api.route-manifest.json',
  },
  {
    file: 'generated/openapi/notary-backend-api.openapi.json',
    apiSurface: 'backend-api',
    manifestFile:
      'sdks/_route-manifests/backend-api/sdkwork-router-notary-backend-api.route-manifest.json',
  },
];

const HTTP_METHODS = new Set(['get', 'post', 'put', 'patch', 'delete', 'head', 'options']);

async function readJson(relativePath) {
  const text = await readFile(path.join(ROOT, relativePath), 'utf8');
  return JSON.parse(text);
}

async function writeJson(relativePath, value) {
  await writeFile(path.join(ROOT, relativePath), `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function routeIndex(manifest) {
  const routesByOperationId = new Map();
  for (const route of manifest.routes ?? []) {
    routesByOperationId.set(route.operationId, route);
  }
  return routesByOperationId;
}

function inferAuthMode(operation) {
  const security = operation.security ?? [];
  const hasDualToken = security.some(
    (entry) => Object.hasOwn(entry, 'AuthToken') && Object.hasOwn(entry, 'AccessToken'),
  );
  return hasDualToken ? 'dual-token' : 'anonymous';
}

async function syncRouteManifests() {
  for (const { file, apiSurface } of ROUTE_MANIFESTS) {
    const manifest = await readJson(file);
    for (const route of manifest.routes ?? []) {
      route.requestContext = 'WebRequestContext';
      route.apiSurface = apiSurface;
    }
    await writeJson(file, manifest);
  }
}

async function syncOpenApiFiles() {
  const manifestRoutes = new Map();
  const manifestPackages = new Map();

  for (const { manifestFile } of OPENAPI_FILES) {
    if (manifestRoutes.has(manifestFile)) {
      continue;
    }
    const manifest = await readJson(manifestFile);
    manifestRoutes.set(manifestFile, routeIndex(manifest));
    manifestPackages.set(manifestFile, manifest.packageName);
  }

  for (const { file, apiSurface, manifestFile } of OPENAPI_FILES) {
    const openapi = await readJson(file);
    const routes = manifestRoutes.get(manifestFile);
    const packageName = manifestPackages.get(manifestFile);

    for (const pathItem of Object.values(openapi.paths ?? {})) {
      for (const [method, operation] of Object.entries(pathItem ?? {})) {
        if (!HTTP_METHODS.has(method) || !operation?.operationId) {
          continue;
        }

        const route = routes.get(operation.operationId);
        operation['x-sdkwork-request-context'] = 'WebRequestContext';
        operation['x-sdkwork-api-surface'] = apiSurface;
        operation['x-sdkwork-auth-mode'] = route?.auth?.mode ?? inferAuthMode(operation);
        operation['x-sdkwork-source-route-crate'] = packageName;
        if (!route?.source?.file) {
          throw new Error(
            `${file} ${operation.operationId} missing route manifest source metadata`,
          );
        }
        operation['x-sdkwork-source'] = route.source.file;
      }
    }

    await writeJson(file, openapi);
  }
}

await syncRouteManifests();
await syncOpenApiFiles();
console.log('Synced Notary route manifests and OpenAPI framework metadata.');
