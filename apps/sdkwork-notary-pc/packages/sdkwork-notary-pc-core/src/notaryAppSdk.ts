import {
  createNotaryApi,
  createNotaryAppClient,
  type CreateNotaryApiOptions,
  type SdkworkAppConfig,
  type SdkworkNotaryAppClient,
} from '@sdkwork/notary-app-sdk';

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

export function createNotaryPcComposedApi(
  options: Partial<Pick<CreateNotaryApiOptions, 'drive' | 'commerce' | 'appbase'>> = {},
) {
  return createNotaryApi({
    notary: getNotaryPcAppSdkClient().notary,
    drive: options.drive ?? {},
    commerce: options.commerce,
    appbase: options.appbase ?? {},
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
}
