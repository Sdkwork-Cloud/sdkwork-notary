#!/usr/bin/env node
/**
 * Generate rich SDKWork route manifests from OpenAPI authorities and routes.rs handler wiring.
 */
import { readFileSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const ROOT = process.cwd();

const SURFACES = [
  {
    apiSurface: 'app-api',
    openapiFile: 'apis/app-api/notary/notary-app-api.openapi.json',
    routesFile: 'crates/sdkwork-routes-notary-app-api/src/routes.rs',
    handlersFile: 'crates/sdkwork-routes-notary-app-api/src/handlers.rs',
    packageName: 'sdkwork-routes-notary-app-api',
    handlerModule: 'sdkwork_routes_notary_app_api::handlers',
    crateRoot: 'crates/sdkwork-routes-notary-app-api',
    crateImport: 'sdkwork_routes_notary_app_api',
    apiAuthority: 'sdkwork-notary-app-api',
    sdkFamily: 'sdkwork-notary-app-sdk',
    prefix: '/app/v3/api',
    outputFile:
      'sdks/_route-manifests/app-api/sdkwork-routes-notary-app-api.route-manifest.json',
    manifestFn: 'sdkwork_notary_app_api_route_manifest',
    manifestRs: 'crates/sdkwork-routes-notary-app-api/src/manifest.rs',
  },
  {
    apiSurface: 'backend-api',
    openapiFile: 'apis/backend-api/notary/notary-backend-api.openapi.json',
    routesFile: 'crates/sdkwork-routes-notary-backend-api/src/routes.rs',
    handlersFile: 'crates/sdkwork-routes-notary-backend-api/src/handlers.rs',
    packageName: 'sdkwork-routes-notary-backend-api',
    handlerModule: 'sdkwork_routes_notary_backend_api::handlers',
    crateRoot: 'crates/sdkwork-routes-notary-backend-api',
    crateImport: 'sdkwork_routes_notary_backend_api',
    apiAuthority: 'sdkwork-notary-backend-api',
    sdkFamily: 'sdkwork-notary-backend-sdk',
    prefix: '/backend/v3/api',
    outputFile:
      'sdks/_route-manifests/backend-api/sdkwork-routes-notary-backend-api.route-manifest.json',
    manifestFn: 'sdkwork_notary_backend_api_route_manifest',
    manifestRs: 'crates/sdkwork-routes-notary-backend-api/src/manifest.rs',
  },
];

const HTTP_METHODS = new Set(['get', 'post', 'put', 'patch', 'delete', 'head', 'options']);

function toOpenApiPath(axumPath) {
  return axumPath.replace(/:([a-z0-9_]+)/g, (_match, segment) => {
    const camel = segment.replace(/_([a-z])/g, (_inner, letter) => letter.toUpperCase());
    return `{${camel}}`;
  });
}

function parseHandlerMap(routesSource) {
  const handlerMap = new Map();

  for (const segment of routesSource.split('.route(').slice(1)) {
    const pathMatch = segment.match(/^\s*"([^"]+)"/);
    if (!pathMatch) {
      continue;
    }

    const openApiPath = toOpenApiPath(pathMatch[1]);
    for (const handlerMatch of segment.matchAll(
      /(get|post|put|patch|delete)\(handlers::([a-z0-9_]+)\)/g,
    )) {
      handlerMap.set(`${handlerMatch[1].toUpperCase()} ${openApiPath}`, handlerMatch[2]);
    }
  }

  return handlerMap;
}

function authFromOperation(operation) {
  const hasDualToken = Array.isArray(operation.security)
    && operation.security.some(
      (entry) => Object.hasOwn(entry, 'AuthToken') && Object.hasOwn(entry, 'AccessToken'),
    );

  return {
    mode: hasDualToken ? 'dual-token' : 'public',
    required: hasDualToken,
    permission: operation['x-sdkwork-permission'] ?? null,
    tenantScope: operation['x-sdkwork-tenant-scope'] ?? null,
    dataScope: operation['x-sdkwork-data-scope'] ?? null,
  };
}

function buildManifest(surface, handlerMap) {
  const openapi = JSON.parse(readFileSync(path.join(ROOT, surface.openapiFile), 'utf8'));
  const routes = [];

  for (const [pathKey, pathItem] of Object.entries(openapi.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!HTTP_METHODS.has(method) || !operation?.operationId) {
        continue;
      }

      const handlerName =
        handlerMap.get(`${method.toUpperCase()} ${pathKey}`)
        ?? throwMissingHandler(surface.packageName, method, pathKey);

      routes.push({
        method: method.toUpperCase(),
        path: pathKey,
        operationId: operation.operationId,
        tags: operation.tags ?? ['notary'],
        auth: authFromOperation(operation),
        handler: {
          module: surface.handlerModule,
          name: handlerName,
        },
        schemas: {
          request: null,
          response: null,
          problem: 'ProblemDetail',
        },
        ownership: {
          owner: 'sdkwork-notary',
          apiAuthority: surface.apiAuthority,
        },
        source: {
          file: `${surface.crateRoot}/src/handlers.rs`,
        },
        requestContext: 'WebRequestContext',
        apiSurface: surface.apiSurface,
      });
    }
  }

  routes.sort((left, right) => left.operationId.localeCompare(right.operationId));

  return {
    schemaVersion: 1,
    kind: 'sdkwork.route.manifest',
    packageName: surface.packageName,
    surface: surface.apiSurface,
    owner: 'sdkwork-notary',
    domain: 'notary',
    capability: 'notary',
    apiAuthority: surface.apiAuthority,
    sdkFamily: surface.sdkFamily,
    prefix: surface.prefix,
    source: {
      crateRoot: surface.crateRoot,
      crateImport: surface.crateImport,
    },
    routes,
  };
}

function throwMissingHandler(packageName, method, pathKey) {
  throw new Error(`${packageName} missing handler wiring for ${method.toUpperCase()} ${pathKey}`);
}

function compactRoutesForEmbeddedManifest(routes) {
  return routes.map((route) => ({
    method: route.method,
    path: route.path,
    operationId: route.operationId,
    requestContext: route.requestContext,
    apiSurface: route.apiSurface,
  }));
}

async function writeEmbeddedManifest(surface, manifest) {
  const manifestSource = await readFile(path.join(ROOT, surface.manifestRs), 'utf8');
  const embedded = JSON.stringify(
    {
      schemaVersion: manifest.schemaVersion,
      kind: manifest.kind,
      packageName: manifest.packageName,
      surface: manifest.surface,
      owner: manifest.owner,
      domain: manifest.domain,
      capability: manifest.capability,
      apiAuthority: manifest.apiAuthority,
      sdkFamily: manifest.sdkFamily,
      prefix: manifest.prefix,
      routes: compactRoutesForEmbeddedManifest(manifest.routes),
    },
    null,
    2,
  );

  const replacement = `pub fn ${surface.manifestFn}() -> &'static str {\n    r#"${embedded}\n"#\n}`;
  const updated = manifestSource.replace(
    new RegExp(`pub fn ${surface.manifestFn}\\(\\) -> &'static str \\{[\\s\\S]*?\\n\\}`, 'm'),
    replacement,
  );

  if (updated === manifestSource) {
    throw new Error(`Failed to update embedded manifest in ${surface.manifestRs}`);
  }

  await writeFile(path.join(ROOT, surface.manifestRs), updated, 'utf8');
}

async function main() {
  for (const surface of SURFACES) {
    const routesSource = await readFile(path.join(ROOT, surface.routesFile), 'utf8');
    const handlerMap = parseHandlerMap(routesSource);
    const manifest = buildManifest(surface, handlerMap);

    await mkdir(path.dirname(path.join(ROOT, surface.outputFile)), { recursive: true });
    await writeFile(
      path.join(ROOT, surface.outputFile),
      `${JSON.stringify(manifest, null, 2)}\n`,
      'utf8',
    );
    await writeEmbeddedManifest(surface, manifest);
    console.log(`Generated ${surface.outputFile} (${manifest.routes.length} routes)`);
  }
}

await main();
