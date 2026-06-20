import {
  bootstrapNotaryH5SdkClients,
  getNotaryH5ComposedApi,
  initNotaryH5AppSdkClient,
} from '@sdkwork/notary-h5-core';

import { resolveEnvironment } from './environment';
import { getTokenManager } from './tokenManager';

export function bootstrapSdkClients() {
  const environment = resolveEnvironment();
  const tokenManager = getTokenManager() ?? undefined;

  initNotaryH5AppSdkClient({
    baseUrl: environment.apiBaseUrl,
    platform: 'h5',
    tokenManager,
    accessToken: tokenManager?.getAccessToken?.(),
  });

  return bootstrapNotaryH5SdkClients();
}

export { getNotaryH5ComposedApi };
