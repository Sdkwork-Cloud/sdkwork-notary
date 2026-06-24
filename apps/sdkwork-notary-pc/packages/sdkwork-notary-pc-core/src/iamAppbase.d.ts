declare module '@sdkwork/iam-contracts' {
  export interface IamAppContext {
    [key: string]: unknown;
  }
}

declare module '@sdkwork/auth-runtime-pc-react' {
  import type { AuthTokenManager } from '@sdkwork/sdk-common';

  export interface SdkworkAppbasePcAuthRuntimeComposition {
    runtime: unknown;
  }

  export interface CreateSdkworkAppbasePcAuthRuntimeOptions {
    app: {
      appId: string;
      deploymentMode: string;
      environment: string;
      platform: string;
    };
    baseUrls: {
      appbaseAppApiBaseUrl: string;
    };
    hooks?: {
      onSessionChanged?: () => void;
    };
    sdkClients: unknown[];
    sessionBridge: {
      clearSession: () => void;
      commitSession: (session: unknown) => unknown;
      readSession: () => unknown;
    };
    tokenManager: AuthTokenManager;
  }

  export function createSdkworkAppbasePcAuthRuntime(
    options: CreateSdkworkAppbasePcAuthRuntimeOptions,
  ): SdkworkAppbasePcAuthRuntimeComposition;
}

declare module '@sdkwork/auth-pc-react' {
  import type { ComponentType, ReactNode } from 'react';

  export interface SdkworkAuthRuntimeConfig {
    [key: string]: unknown;
  }

  export interface SdkworkAuthAppearanceConfig {
    [key: string]: unknown;
  }

  export interface SdkworkIamRuntimeAuthRuntimeLike {
    service: {
      auth: {
        sessions: {
          current: {
            retrieve(): Promise<unknown>;
            delete(): Promise<unknown>;
          };
        };
      };
    };
  }

  export interface SdkworkIamAuthRoutesProps {
    appearance: SdkworkAuthAppearanceConfig;
    basePath: string;
    getRuntime: () => SdkworkIamRuntimeAuthRuntimeLike;
    homePath: string;
    locale?: string | null;
    runtimeConfig: SdkworkAuthRuntimeConfig;
    viewportMode?: string;
  }

  export const SdkworkIamAuthRoutes: ComponentType<SdkworkIamAuthRoutesProps>;
}
