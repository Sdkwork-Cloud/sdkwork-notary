import { isBlank } from '@sdkwork/utils/string';

export interface NotaryPcEnvironment {
  apiBaseUrl: string;
  backendApiBaseUrl: string;
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
  const backendApiBaseUrl =
    import.meta.env.VITE_SDKWORK_NOTARY_APPLICATION_BACKEND_HTTP_URL
    ?? import.meta.env.VITE_SDKWORK_NOTARY_PLATFORM_API_GATEWAY_HTTP_URL;

  const resolvedApiBaseUrl = typeof apiBaseUrl === 'string' ? apiBaseUrl.trim() : '';
  const resolvedBackendApiBaseUrl = typeof backendApiBaseUrl === 'string'
    ? backendApiBaseUrl.trim()
    : '';
  if (isBlank(resolvedApiBaseUrl) || isBlank(resolvedBackendApiBaseUrl)) {
    if (import.meta.env.PROD) {
      throw new Error(
        'Notary PC runtime config requires public and backend API base URLs. Configure VITE_SDKWORK_NOTARY_APPLICATION_PUBLIC_HTTP_URL and VITE_SDKWORK_NOTARY_APPLICATION_BACKEND_HTTP_URL, or use VITE_SDKWORK_NOTARY_PLATFORM_API_GATEWAY_HTTP_URL for a shared gateway.',
      );
    }
    return {
      apiBaseUrl: 'http://127.0.0.1:18085',
      backendApiBaseUrl: 'http://127.0.0.1:18086',
      profile: import.meta.env.MODE ?? 'development',
    };
  }

  return {
    apiBaseUrl: defaultIfBlank(resolvedApiBaseUrl, 'http://127.0.0.1:18085'),
    backendApiBaseUrl: defaultIfBlank(resolvedBackendApiBaseUrl, 'http://127.0.0.1:18086'),
    profile: import.meta.env.MODE ?? 'development',
  };
}
