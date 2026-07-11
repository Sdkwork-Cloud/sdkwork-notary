import {
  createClient,
  type SdkworkBackendConfig,
  type SdkworkNotaryBackendClient,
} from '@sdkwork/notary-backend-sdk';
import { getNotaryPcGlobalTokenManager } from '@sdkwork/notary-pc-core';
import { trim } from '@sdkwork/utils';

export type NotaryPcAdminBackendSdkConfig = Omit<
  SdkworkBackendConfig,
  'accessToken' | 'authToken' | 'baseUrl' | 'tokenManager'
> & {
  baseUrl: string;
};

let backendSdkClient: SdkworkNotaryBackendClient | null = null;

function normalizeBackendBaseUrl(value: string): string {
  const normalized = trim(value).replace(/\/+$/u, '');
  if (!normalized) {
    throw new Error('Notary PC admin backend base URL is required.');
  }
  return normalized;
}

export function createNotaryPcAdminBackendSdkClient(
  config: NotaryPcAdminBackendSdkConfig,
): SdkworkNotaryBackendClient {
  return createClient({
    ...config,
    baseUrl: normalizeBackendBaseUrl(config.baseUrl),
    platform: config.platform ?? 'pc',
    tokenManager: getNotaryPcGlobalTokenManager(),
  });
}

export function initNotaryPcAdminBackendSdkClient(
  config: NotaryPcAdminBackendSdkConfig,
): SdkworkNotaryBackendClient {
  backendSdkClient = createNotaryPcAdminBackendSdkClient(config);
  return backendSdkClient;
}

export function getNotaryPcAdminBackendSdkClient(): SdkworkNotaryBackendClient {
  if (!backendSdkClient) {
    throw new Error(
      'Notary PC admin backend SDK client is not initialized. Call initNotaryPcAdminBackendSdkClient first.',
    );
  }
  return backendSdkClient;
}

export function resetNotaryPcAdminBackendSdkClient(): void {
  backendSdkClient = null;
}
