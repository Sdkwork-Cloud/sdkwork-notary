import {
  createClient,
  type SdkworkAppClient as SdkworkAppbaseAppClient,
  type SdkworkAppConfig,
} from '@sdkwork/iam-app-sdk';

let appbaseAppSdkClient: SdkworkAppbaseAppClient | null = null;

export function initNotaryPcAppbaseAppSdkClient(config: SdkworkAppConfig): SdkworkAppbaseAppClient {
  appbaseAppSdkClient = createClient({
    ...config,
    platform: config.platform ?? 'pc',
  });
  return appbaseAppSdkClient;
}

export function getNotaryPcAppbaseAppSdkClient(): SdkworkAppbaseAppClient {
  if (!appbaseAppSdkClient) {
    throw new Error(
      'Notary PC appbase app SDK client is not initialized. Call initNotaryPcAppbaseAppSdkClient first.',
    );
  }
  return appbaseAppSdkClient;
}

export function resetNotaryPcAppbaseAppSdkClient(): void {
  appbaseAppSdkClient = null;
}
