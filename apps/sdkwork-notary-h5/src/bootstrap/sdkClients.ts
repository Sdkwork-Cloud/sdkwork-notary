import {
  bootstrapNotaryH5SdkClients,
  getNotaryH5AppSdkClient,
  getNotaryH5AppbaseAppSdkClient,
  getNotaryH5DriveAppSdkClient,
  initNotaryH5AppSdkClient,
  initNotaryH5AppbaseAppSdkClient,
  initNotaryH5DriveAppSdkClient,
  registerNotaryH5SdkClientRefresh,
} from '@sdkwork/notary-h5-core';

import { resolveEnvironment } from './environment';
import { getTokenManager } from './tokenManager';

export function bootstrapSdkClients() {
  const environment = resolveEnvironment();
  const tokenManager = getTokenManager() ?? undefined;
  const sdkClientConfig = {
    baseUrl: environment.apiBaseUrl,
    platform: 'h5' as const,
    tokenManager,
    accessToken: tokenManager?.getAccessToken?.(),
  };

  initNotaryH5AppSdkClient(sdkClientConfig);
  initNotaryH5DriveAppSdkClient(sdkClientConfig);
  initNotaryH5AppbaseAppSdkClient(sdkClientConfig);

  return bootstrapNotaryH5SdkClients();
}

registerNotaryH5SdkClientRefresh(bootstrapSdkClients);

export { getNotaryH5ComposedApi } from '@sdkwork/notary-h5-core';
