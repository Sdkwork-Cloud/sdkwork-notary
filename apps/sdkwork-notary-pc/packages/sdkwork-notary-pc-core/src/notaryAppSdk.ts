import {
  createNotaryApi,
  createNotaryAppClient,
  type AppbaseAppSdkPort,
  type CreateNotaryApiOptions,
  type DriveAppSdkPort,
  type SdkworkAppConfig,
  type SdkworkNotaryAppClient,
} from '@sdkwork/notary-app-sdk';

import { getNotaryPcAppbaseAppSdkClient, resetNotaryPcAppbaseAppSdkClient } from './appbaseAppSdk';
import { getNotaryPcDriveAppSdkClient, resetNotaryPcDriveAppSdkClient } from './driveAppSdk';

let notaryAppSdkClient: SdkworkNotaryAppClient | null = null;
let notaryComposedApi: ReturnType<typeof createNotaryApi> | null = null;

export function initNotaryPcAppSdkClient(config: SdkworkAppConfig): SdkworkNotaryAppClient {
  notaryAppSdkClient = createNotaryAppClient({
    ...config,
    platform: config.platform ?? 'pc',
  });
  notaryComposedApi = null;
  return notaryAppSdkClient;
}

export function getNotaryPcAppSdkClient(): SdkworkNotaryAppClient {
  if (!notaryAppSdkClient) {
    throw new Error('Notary PC app SDK client is not initialized. Call initNotaryPcAppSdkClient first.');
  }
  return notaryAppSdkClient;
}

function resolveComposedDependencyClients(
  options: Partial<Pick<CreateNotaryApiOptions, 'drive' | 'commerce' | 'appbase'>>,
): Pick<CreateNotaryApiOptions, 'drive' | 'commerce' | 'appbase'> {
  return {
    drive: (options.drive ?? getNotaryPcDriveAppSdkClient()) as DriveAppSdkPort,
    commerce: options.commerce,
    appbase: (options.appbase ?? getNotaryPcAppbaseAppSdkClient()) as AppbaseAppSdkPort,
  };
}

export function createNotaryPcComposedApi(
  options: Partial<Pick<CreateNotaryApiOptions, 'drive' | 'commerce' | 'appbase'>> = {},
) {
  const dependencies = resolveComposedDependencyClients(options);
  return createNotaryApi({
    notary: getNotaryPcAppSdkClient().notary,
    drive: dependencies.drive,
    commerce: dependencies.commerce,
    appbase: dependencies.appbase,
  });
}

export function getNotaryPcComposedApi() {
  if (!notaryComposedApi) {
    notaryComposedApi = createNotaryPcComposedApi();
  }
  return notaryComposedApi;
}

export function bootstrapNotaryPcSdkClients() {
  return {
    app: getNotaryPcAppSdkClient(),
    api: getNotaryPcComposedApi(),
  };
}

export function resetNotaryPcSdkClients(): void {
  notaryAppSdkClient = null;
  notaryComposedApi = null;
  resetNotaryPcDriveAppSdkClient();
  resetNotaryPcAppbaseAppSdkClient();
}
