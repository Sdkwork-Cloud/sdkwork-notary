declare module '@sdkwork/notary-app-sdk' {
  export interface SdkworkAppConfig {
    baseUrl?: string;
    platform?: string;
    accessToken?: string;
    authToken?: string;
    tokenManager?: unknown;
  }

  export interface SdkworkNotaryAppClient {
    notary: unknown;
  }

  export interface CreateNotaryApiOptions {
    notary: unknown;
    drive: unknown;
    commerce?: unknown;
    appbase: unknown;
  }

  export function createNotaryAppClient(config: SdkworkAppConfig): SdkworkNotaryAppClient;
  export function createNotaryApi(options: CreateNotaryApiOptions): NotaryComposedApi;

  export interface NotaryComposedApi {
    getDashboardStatistics(): Promise<unknown>;
    listCases(input?: unknown): Promise<unknown>;
  }
}
