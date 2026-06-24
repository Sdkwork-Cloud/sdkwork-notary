import {
  createDriveAppClient,
  type SdkworkAppConfig,
  type SdkworkDriveAppClient,
} from '@sdkwork/drive-app-sdk';

let driveAppSdkClient: SdkworkDriveAppClient | null = null;

export function initNotaryH5DriveAppSdkClient(config: SdkworkAppConfig): SdkworkDriveAppClient {
  driveAppSdkClient = createDriveAppClient({
    ...config,
    platform: config.platform ?? 'h5',
  });
  return driveAppSdkClient;
}

export function getNotaryH5DriveAppSdkClient(): SdkworkDriveAppClient {
  if (!driveAppSdkClient) {
    throw new Error(
      'Notary H5 drive app SDK client is not initialized. Call initNotaryH5DriveAppSdkClient first.',
    );
  }
  return driveAppSdkClient;
}

export function resetNotaryH5DriveAppSdkClient(): void {
  driveAppSdkClient = null;
}
