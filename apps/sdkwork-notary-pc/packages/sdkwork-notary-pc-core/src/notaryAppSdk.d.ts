declare module '@sdkwork/notary-app-sdk' {
  export interface SdkworkAppConfig {
    baseUrl?: string;
    platform?: string;
    accessToken?: string;
    authToken?: string;
    tokenManager?: unknown;
    requestContextInterceptors?: unknown;
  }

  export interface SdkworkNotaryAppClient {
    notary: {
      access: {
        retrieve(): Promise<unknown>;
      };
      [resource: string]: {
        [action: string]: (...args: unknown[]) => Promise<unknown>;
      };
    };
  }

  export interface CreateNotaryApiOptions {
    notary: unknown;
    drive: unknown;
    commerce?: unknown;
    appbase: unknown;
  }

  export type NotaryComposedApi = {
    getAccess(...args: unknown[]): Promise<unknown>;
    getDashboardStatistics(...args: unknown[]): Promise<unknown>;
    getMonthlyReport(...args: unknown[]): Promise<unknown>;
    listMatters(...args: unknown[]): Promise<unknown>;
    listCases(...args: unknown[]): Promise<unknown>;
    getCase(...args: unknown[]): Promise<unknown>;
    retrieveCase(...args: unknown[]): Promise<unknown>;
    createCase(...args: unknown[]): Promise<unknown>;
    updateCase(...args: unknown[]): Promise<unknown>;
    listStaff(...args: unknown[]): Promise<unknown>;
    assignCase(...args: unknown[]): Promise<unknown>;
    listCaseFiles(...args: unknown[]): Promise<unknown>;
    createCaseFile(...args: unknown[]): Promise<unknown>;
    deleteCaseFile(...args: unknown[]): Promise<unknown>;
    createCaseFileDownloadUrl(...args: unknown[]): Promise<unknown>;
    createDownloadPackage(...args: unknown[]): Promise<unknown>;
    listParties(...args: unknown[]): Promise<unknown>;
    createParty(...args: unknown[]): Promise<unknown>;
    addParty(...args: unknown[]): Promise<unknown>;
    updateParty(...args: unknown[]): Promise<unknown>;
    deleteParty(...args: unknown[]): Promise<unknown>;
    createPartyVideoInvite(...args: unknown[]): Promise<unknown>;
    createPartySignatureInvite(...args: unknown[]): Promise<unknown>;
    attachPartySignature(...args: unknown[]): Promise<unknown>;
    uploadCaseFile(...args: unknown[]): Promise<unknown>;
    acceptCase(...args: unknown[]): Promise<unknown>;
    rejectCase(...args: unknown[]): Promise<unknown>;
    completeCase(...args: unknown[]): Promise<unknown>;
    listCaseEvents(...args: unknown[]): Promise<unknown>;
  };

  export function createNotaryAppClient(config: SdkworkAppConfig): SdkworkNotaryAppClient;
  export function createNotaryApi(options: CreateNotaryApiOptions): NotaryComposedApi;
}
