import { isBlank } from '@sdkwork/utils/string';

export interface NotaryPcEnvironment {
  apiBaseUrl: string;
  profile: string;
}

function defaultIfBlank(value: string | null | undefined, defaultValue: string): string {
  return isBlank(value) ? defaultValue : value!.trim();
}

export function resolveEnvironment(): NotaryPcEnvironment {
  const apiBaseUrl =
    import.meta.env.VITE_SDKWORK_NOTARY_APPLICATION_PUBLIC_HTTP_URL
    ?? import.meta.env.VITE_SDKWORK_NOTARY_PLATFORM_API_GATEWAY_HTTP_URL
    ?? import.meta.env.VITE_SDKWORK_NOTARY_APP_HTTP_URL;

  const resolved = typeof apiBaseUrl === 'string' ? apiBaseUrl.trim() : '';
  if (isBlank(resolved)) {
    if (import.meta.env.PROD) {
      throw new Error(
        'Notary PC runtime config is missing a public API base URL. Configure VITE_SDKWORK_NOTARY_APPLICATION_PUBLIC_HTTP_URL or VITE_SDKWORK_NOTARY_PLATFORM_API_GATEWAY_HTTP_URL.',
      );
    }
    return {
      apiBaseUrl: 'http://127.0.0.1:18085',
      profile: import.meta.env.MODE ?? 'development',
    };
  }

  return {
    apiBaseUrl: defaultIfBlank(resolved, 'http://127.0.0.1:18085'),
    profile: import.meta.env.MODE ?? 'development',
  };
}
