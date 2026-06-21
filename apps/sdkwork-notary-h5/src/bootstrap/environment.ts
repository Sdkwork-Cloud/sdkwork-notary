import { isBlank } from '@sdkwork/utils/string';

export interface NotaryH5Environment {
  apiBaseUrl: string;
  profile: string;
}

function defaultIfBlank(value: string | null | undefined, defaultValue: string): string {
  return isBlank(value) ? defaultValue : value!.trim();
}

export function resolveEnvironment(): NotaryH5Environment {
  const apiBaseUrl = import.meta.env.VITE_SDKWORK_NOTARY_APPLICATION_PUBLIC_HTTP_URL
    ?? import.meta.env.VITE_SDKWORK_NOTARY_APP_HTTP_URL;

  return {
    apiBaseUrl: defaultIfBlank(
      typeof apiBaseUrl === 'string' ? apiBaseUrl : undefined,
      'http://127.0.0.1:18085',
    ),
    profile: import.meta.env.MODE ?? 'development',
  };
}
