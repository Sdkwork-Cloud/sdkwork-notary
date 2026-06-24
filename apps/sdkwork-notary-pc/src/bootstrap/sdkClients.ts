import {
  bootstrapNotaryPcSdkClients,
  getNotaryPcAppSdkClient,
  getNotaryPcAppbaseAppSdkClient,
  getNotaryPcDriveAppSdkClient,
  initNotaryPcAppSdkClient,
  initNotaryPcAppbaseAppSdkClient,
  initNotaryPcDriveAppSdkClient,
  registerNotaryPcSdkClientRefresh,
} from '@sdkwork/notary-pc-core';
import { createDefaultBrowserHostAdapter } from '@sdkwork/notary-pc-commons';
import { configureNotaryPcRuntime } from '@sdkwork/notary-pc-notary';

import { resolveEnvironment } from './environment';
import { getTokenManager } from './tokenManager';

export function bootstrapSdkClients() {
  const environment = resolveEnvironment();
  const tokenManager = getTokenManager() ?? undefined;
  const sdkClientConfig = {
    baseUrl: environment.apiBaseUrl,
    platform: 'pc' as const,
    tokenManager,
    accessToken: tokenManager?.getAccessToken?.(),
  };

  initNotaryPcAppSdkClient(sdkClientConfig);
  initNotaryPcDriveAppSdkClient(sdkClientConfig);
  initNotaryPcAppbaseAppSdkClient(sdkClientConfig);

  configureNotaryPcRuntime({
    host: createDefaultBrowserHostAdapter(),
    sdkPorts: {
      getNotaryClient: getNotaryPcAppSdkClient,
      getDriveClient: getNotaryPcDriveAppSdkClient,
      getAppbaseClient: getNotaryPcAppbaseAppSdkClient,
    },
  });

  return bootstrapNotaryPcSdkClients();
}

registerNotaryPcSdkClientRefresh(bootstrapSdkClients);
