import {
  createDriveAppClient,
  type SdkworkAppConfig,
  type SdkworkDriveAppClient,
} from '@sdkwork/drive-app-sdk';

let driveAppSdkClient: SdkworkDriveAppClient | null = null;

export function initNotaryPcDriveAppSdkClient(config: SdkworkAppConfig): SdkworkDriveAppClient {
  driveAppSdkClient = createDriveAppClient({
    ...config,
    platform: config.platform ?? 'pc',
  });
  return driveAppSdkClient;
}

export function getNotaryPcDriveAppSdkClient(): SdkworkDriveAppClient {
  if (!driveAppSdkClient) {
    throw new Error(
      'Notary PC drive app SDK client is not initialized. Call initNotaryPcDriveAppSdkClient first.',
    );
  }
  return driveAppSdkClient;
}

export function resetNotaryPcDriveAppSdkClient(): void {
  driveAppSdkClient = null;
}
