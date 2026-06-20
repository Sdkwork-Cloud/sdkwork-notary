export interface NotaryH5Environment {
  apiBaseUrl: string;
  profile: string;
}

export function resolveEnvironment(): NotaryH5Environment {
  const apiBaseUrl = import.meta.env.VITE_SDKWORK_NOTARY_APP_HTTP_URL;
  return {
    apiBaseUrl: typeof apiBaseUrl === 'string' && apiBaseUrl.trim().length > 0
      ? apiBaseUrl.trim()
      : 'http://127.0.0.1:18085',
    profile: import.meta.env.MODE ?? 'development',
  };
}
