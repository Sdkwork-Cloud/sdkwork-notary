import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  applyDevelopmentLocalGatewayBinding,
  buildProfileId,
  createTopologyRuntime,
  isTcpPortReachable,
  loadTopologySpec,
  normalizeText,
  waitForHttpHealthy,
} from '@sdkwork/app-topology';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const REPO_ROOT = path.resolve(__dirname, '..', '..');
export const SPEC_PATH = path.join(REPO_ROOT, 'specs/topology.spec.json');

const spec = loadTopologySpec(SPEC_PATH);
const runtime = createTopologyRuntime(spec, REPO_ROOT);

export const DEFAULT_DEV_PROFILE_ID = runtime.defaults.developmentProfileId;
export const DEFAULT_PRODUCTION_PROFILE_ID = runtime.defaults.productionProfileId;

export function resolveDevProfileId(deploymentProfile, environment = 'development') {
  runtime.assertHosting(deploymentProfile);
  runtime.assertEnvironment(environment);
  return buildProfileId(deploymentProfile, environment);
}

export function resolveDefaultAppSdkBaseUrl(profileEnv = {}) {
  return (
    profileEnv.SDKWORK_NOTARY_APPLICATION_PUBLIC_HTTP_URL
    ?? profileEnv.VITE_SDKWORK_NOTARY_APPLICATION_PUBLIC_HTTP_URL
    ?? 'http://127.0.0.1:18085'
  );
}

export function resolveDefaultBackendSdkBaseUrl(profileEnv = {}) {
  return (
    profileEnv.SDKWORK_NOTARY_APPLICATION_BACKEND_HTTP_URL
    ?? profileEnv.VITE_SDKWORK_NOTARY_APPLICATION_BACKEND_HTTP_URL
    ?? 'http://127.0.0.1:18086'
  );
}

export const loadProfile = (profileId) => applyDevelopmentLocalGatewayBinding(
  runtime.loadProfile(profileId),
  { profileId },
);
export const applyProfileEnv = runtime.applyProfileEnv;
export const mergeRuntimeEnv = runtime.mergeRuntimeEnv;
export const loadEnvFile = runtime.loadEnvFile;
export const assertHosting = runtime.assertHosting;
export const assertEnvironment = runtime.assertEnvironment;
export const resolveSurfaceHttpUrl = runtime.resolveSurfaceHttpUrl.bind(runtime);
export const resolveSurfaceBind = runtime.resolveSurfaceBind.bind(runtime);
export const shouldAutostartGateway = runtime.shouldAutostartGateway;
export const resolveGatewayBind = runtime.resolveGatewayBind;
export const resolveGatewayBaseUrl = runtime.resolveGatewayBaseUrl;
export const resolveIamDevEnv = runtime.resolveIamDevEnv;
export const listOrchestrationProcesses = runtime.listOrchestrationProcesses;
export const listHealthSurfaces = runtime.listHealthSurfaces;

export { buildProfileId, normalizeText, isTcpPortReachable, waitForHttpHealthy, spec, runtime };
