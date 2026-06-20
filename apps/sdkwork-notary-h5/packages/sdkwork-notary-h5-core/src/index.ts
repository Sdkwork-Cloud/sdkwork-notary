import {
  createNotaryApi,
  createNotaryAppClient,
  type CreateNotaryApiOptions,
  type SdkworkAppConfig,
  type SdkworkNotaryAppClient,
} from '@sdkwork/notary-app-sdk';

let notaryAppSdkClient: SdkworkNotaryAppClient | null = null;
let notaryComposedApi: ReturnType<typeof createNotaryApi> | null = null;

export function initNotaryH5AppSdkClient(config: SdkworkAppConfig): SdkworkNotaryAppClient {
  notaryAppSdkClient = createNotaryAppClient({
    ...config,
    platform: config.platform ?? 'h5',
  });
  notaryComposedApi = null;
  return notaryAppSdkClient;
}

export function getNotaryH5AppSdkClient(): SdkworkNotaryAppClient {
  if (!notaryAppSdkClient) {
    throw new Error('Notary H5 app SDK client is not initialized. Call initNotaryH5AppSdkClient first.');
  }
  return notaryAppSdkClient;
}

export function createNotaryH5ComposedApi(
  options: Partial<Pick<CreateNotaryApiOptions, 'drive' | 'commerce' | 'appbase'>> = {},
) {
  return createNotaryApi({
    notary: getNotaryH5AppSdkClient().notary,
    drive: options.drive ?? {},
    commerce: options.commerce,
    appbase: options.appbase ?? {},
  });
}

export function getNotaryH5ComposedApi() {
  if (!notaryComposedApi) {
    notaryComposedApi = createNotaryH5ComposedApi();
  }
  return notaryComposedApi;
}

export function bootstrapNotaryH5SdkClients() {
  return {
    app: getNotaryH5AppSdkClient(),
    api: getNotaryH5ComposedApi(),
  };
}

export function resetNotaryH5SdkClients(): void {
  notaryAppSdkClient = null;
  notaryComposedApi = null;
}
