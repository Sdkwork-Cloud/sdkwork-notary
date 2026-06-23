import {
  bootstrapNotaryPcSdkClients,
  getNotaryPcAppSdkClient,
  initNotaryPcAppSdkClient,
} from '@sdkwork/notary-pc-core';
import { configureNotaryPcRuntime } from '@sdkwork/notary-pc-notary';
import { createDefaultBrowserHostAdapter } from '@sdkwork/notary-pc-commons';

import { resolveEnvironment } from './environment';
import { getTokenManager } from './tokenManager';

export function bootstrapSdkClients() {
  const environment = resolveEnvironment();
  const tokenManager = getTokenManager() ?? undefined;

  initNotaryPcAppSdkClient({
    baseUrl: environment.apiBaseUrl,
    platform: 'pc',
    tokenManager,
    accessToken: tokenManager?.getAccessToken?.(),
  });

  configureNotaryPcRuntime({
    host: createDefaultBrowserHostAdapter(),
    sdkPorts: {
      getNotaryClient: getNotaryPcAppSdkClient,
      getDriveClient: () => ({}),
      getAppbaseClient: () => ({}),
    },
  });

  return bootstrapNotaryPcSdkClients();
}
