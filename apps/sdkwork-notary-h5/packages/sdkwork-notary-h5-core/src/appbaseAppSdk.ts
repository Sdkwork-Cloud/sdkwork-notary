import {
  createClient,
  type SdkworkAppClient as SdkworkAppbaseAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/iam-app-sdk';

let appbaseAppSdkClient: SdkworkAppbaseAppClient | null = null;

export function initNotaryH5AppbaseAppSdkClient(config: SdkworkAppConfig): SdkworkAppbaseAppClient {
  appbaseAppSdkClient = createClient({
    ...config,
    platform: config.platform ?? 'h5',
  });
  return appbaseAppSdkClient;
}

export function getNotaryH5AppbaseAppSdkClient(): SdkworkAppbaseAppClient {
  if (!appbaseAppSdkClient) {
    throw new Error(
      'Notary H5 appbase app SDK client is not initialized. Call initNotaryH5AppbaseAppSdkClient first.',
    );
  }
  return appbaseAppSdkClient;
}

export function resetNotaryH5AppbaseAppSdkClient(): void {
  appbaseAppSdkClient = null;
}
