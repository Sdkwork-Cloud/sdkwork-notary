declare module '@sdkwork/drive-app-sdk' {
  import type { SdkworkAppConfig } from '@sdkwork/notary-app-sdk';

  export type { SdkworkAppConfig };

  export interface SdkworkDriveAppClient {
    [resource: string]: {
      [action: string]: (...args: unknown[]) => Promise<unknown>;
    };
  }

  export function createDriveAppClient(config: SdkworkAppConfig): SdkworkDriveAppClient;
}

declare module '@sdkwork/iam-app-sdk' {
  import type { SdkworkAppConfig } from '@sdkwork/notary-app-sdk';

  export type { SdkworkAppConfig };

  export interface SdkworkAppClient {
    [resource: string]: {
      [action: string]: (...args: unknown[]) => Promise<unknown>;
    };
  }

  export function createClient(config: SdkworkAppConfig): SdkworkAppClient;
}
