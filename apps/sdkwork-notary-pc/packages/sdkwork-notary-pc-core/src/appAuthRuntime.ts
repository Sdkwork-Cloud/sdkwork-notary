import {
  createSdkworkAppbasePcAuthRuntime,
  type SdkworkAppbasePcAuthRuntimeComposition,
} from '@sdkwork/auth-runtime-pc-react';
import type {
  SdkworkAuthAppearanceConfig,
  SdkworkAuthRuntimeConfig,
  SdkworkIamRuntimeAuthRuntimeLike,
} from '@sdkwork/auth-pc-react';

import {
  getNotaryPcAppSdkClient,
} from './notaryAppSdk';
import {
  getNotaryPcAppbaseAppSdkClient,
} from './appbaseAppSdk';
import {
  getNotaryPcDriveAppSdkClient,
} from './driveAppSdk';
import {
  applyNotaryPcSessionTokens,
  clearNotaryPcSessionTokens,
  getNotaryPcGlobalTokenManager,
  readNotaryPcSessionTokens,
  type NotaryPcSession,
} from './session';
import {
  refreshAuthenticatedNotaryPcSdkClients,
  resetAuthenticatedNotaryPcSdkClients,
} from './sdkSessionLifecycle';

type IamEnvironment = 'dev' | 'prod' | 'test';
type IamDeploymentMode = 'local' | 'private' | 'saas';

let notaryPcIamRuntimeComposition: SdkworkAppbasePcAuthRuntimeComposition | null = null;

function readEnvValue(...keys: string[]): string | undefined {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | boolean | undefined>;
  };

  for (const key of keys) {
    const value = meta.env?.[key];
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }

  return undefined;
}

function resolveIamEnvironment(): IamEnvironment {
  const value = readEnvValue('VITE_SDKWORK_NOTARY_IAM_ENVIRONMENT', 'VITE_SDKWORK_IAM_ENVIRONMENT');
  return value === 'prod' || value === 'production'
    ? 'prod'
    : value === 'test'
      ? 'test'
      : 'dev';
}

function resolveIamDeploymentMode(): IamDeploymentMode {
  const value = readEnvValue('VITE_SDKWORK_NOTARY_DEPLOYMENT_MODE', 'VITE_SDKWORK_DEPLOYMENT_MODE');
  return value === 'local' || value === 'private' || value === 'saas'
    ? value
    : 'saas';
}

function resolveAppbaseApiBaseUrl(): string {
  return readEnvValue(
    'VITE_SDKWORK_NOTARY_APPLICATION_PUBLIC_HTTP_URL',
    'VITE_SDKWORK_NOTARY_PLATFORM_API_GATEWAY_HTTP_URL',
    'VITE_SDKWORK_NOTARY_APP_HTTP_URL',
  ) ?? 'http://127.0.0.1:18085';
}

function createNotaryPcIamRuntime(): SdkworkAppbasePcAuthRuntimeComposition {
  return createSdkworkAppbasePcAuthRuntime({
    app: {
      appId: 'sdkwork-notary-pc',
      deploymentMode: resolveIamDeploymentMode(),
      environment: resolveIamEnvironment(),
      platform: 'pc',
    },
    baseUrls: {
      appbaseAppApiBaseUrl: resolveAppbaseApiBaseUrl(),
    },
    hooks: {
      onSessionChanged: () => {
        resetAuthenticatedNotaryPcSdkClients();
        resetNotaryPcIamRuntime();
        refreshAuthenticatedNotaryPcSdkClients();
      },
    },
    sdkClients: [
      getNotaryPcAppSdkClient(),
      getNotaryPcDriveAppSdkClient(),
      getNotaryPcAppbaseAppSdkClient(),
    ],
    sessionBridge: {
      clearSession: clearNotaryPcSessionTokens,
      commitSession: (session) => applyNotaryPcSessionTokens(session as NotaryPcSession),
      readSession: readNotaryPcSessionTokens,
    },
    tokenManager: getNotaryPcGlobalTokenManager(),
  });
}

export function getNotaryPcIamRuntime(): SdkworkIamRuntimeAuthRuntimeLike {
  if (!notaryPcIamRuntimeComposition) {
    notaryPcIamRuntimeComposition = createNotaryPcIamRuntime();
  }

  return notaryPcIamRuntimeComposition.runtime as SdkworkIamRuntimeAuthRuntimeLike;
}

export function resetNotaryPcIamRuntime(): void {
  notaryPcIamRuntimeComposition = null;
}

export function clearNotaryPcIamRuntimeSession(): void {
  clearNotaryPcSessionTokens();
  resetAuthenticatedNotaryPcSdkClients();
  resetNotaryPcIamRuntime();
}

export function resolveNotaryPcAuthRuntimeConfig(): SdkworkAuthRuntimeConfig {
  return {
    leftRailMode: 'qr-only',
    loginMethods: ['password'],
    oauthLoginEnabled: false,
    oauthProviders: [],
    qrLoginEnabled: true,
    recoveryMethods: ['email', 'phone'],
    registerMethods: ['email', 'phone'],
    verificationPolicy: {
      emailCodeLoginEnabled: false,
      emailRegistrationVerificationRequired: false,
      phoneCodeLoginEnabled: false,
      phoneRegistrationVerificationRequired: false,
    },
  };
}

export function resolveNotaryPcAuthAppearance(): SdkworkAuthAppearanceConfig {
  return {
    pageClassName: 'notary-pc-auth-page',
    shellClassName: 'notary-pc-auth-shell',
    theme: {
      pageBackgroundColor: '#0f1115',
      contentBackgroundColor: '#171a21',
      contentTextColor: '#f3f4f6',
      titleColor: '#ffffff',
      descriptionColor: '#9ca3af',
      fieldBackgroundColor: '#111827',
      fieldTextColor: '#f9fafb',
      fieldPlaceholderColor: '#6b7280',
    },
  };
}
