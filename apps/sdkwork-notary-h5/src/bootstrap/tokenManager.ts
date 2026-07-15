import {
  createTokenManager,
  type AuthTokenManager,
  type AuthTokens,
} from '@sdkwork/sdk-common';

import {
  refreshAuthenticatedNotaryH5SdkClients,
  resetAuthenticatedNotaryH5SdkClients,
} from '@sdkwork/notary-h5-core';

const ACCESS_TOKEN_KEY = 'sdkwork.accessToken';
const AUTH_TOKEN_KEY = 'sdkwork.authToken';
const REFRESH_TOKEN_KEY = 'sdkwork.refreshToken';

let tokenManager: AuthTokenManager | null = null;

function readDevBootstrapAccessToken(): string | undefined {
  const nodeProcess = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process;
  const value = (nodeProcess?.env?.SDKWORK_ACCESS_TOKEN ?? '').trim();
  return value.length > 0 ? value : undefined;
}

function readPersistedTokens(): AuthTokens | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const accessToken = window.localStorage.getItem(ACCESS_TOKEN_KEY)
    ?? window.sessionStorage.getItem(ACCESS_TOKEN_KEY)
    ?? undefined;
  const authToken = window.localStorage.getItem(AUTH_TOKEN_KEY)
    ?? window.sessionStorage.getItem(AUTH_TOKEN_KEY)
    ?? undefined;
  const refreshToken = window.localStorage.getItem(REFRESH_TOKEN_KEY)
    ?? window.sessionStorage.getItem(REFRESH_TOKEN_KEY)
    ?? undefined;

  if (!accessToken && !authToken) {
    return undefined;
  }

  return { accessToken, authToken, refreshToken };
}

function readInitialTokens(): AuthTokens | undefined {
  const devAccessToken = readDevBootstrapAccessToken();
  if (devAccessToken) {
    return { accessToken: devAccessToken };
  }

  return readPersistedTokens();
}

function persistTokens(tokens: AuthTokens): void {
  if (typeof window === 'undefined') {
    return;
  }

  if (tokens.accessToken) {
    window.localStorage.setItem(ACCESS_TOKEN_KEY, tokens.accessToken);
  } else {
    window.localStorage.removeItem(ACCESS_TOKEN_KEY);
  }

  if (tokens.authToken) {
    window.localStorage.setItem(AUTH_TOKEN_KEY, tokens.authToken);
  } else {
    window.localStorage.removeItem(AUTH_TOKEN_KEY);
  }
  if (tokens.refreshToken) {
    window.localStorage.setItem(REFRESH_TOKEN_KEY, tokens.refreshToken);
  } else {
    window.localStorage.removeItem(REFRESH_TOKEN_KEY);
  }
  window.sessionStorage.removeItem(ACCESS_TOKEN_KEY);
  window.sessionStorage.removeItem(AUTH_TOKEN_KEY);
  window.sessionStorage.removeItem(REFRESH_TOKEN_KEY);
}

function clearPersistedTokens(): void {
  if (typeof window === 'undefined') {
    return;
  }

  window.localStorage.removeItem(ACCESS_TOKEN_KEY);
  window.localStorage.removeItem(AUTH_TOKEN_KEY);
  window.localStorage.removeItem(REFRESH_TOKEN_KEY);
  window.sessionStorage.removeItem(ACCESS_TOKEN_KEY);
  window.sessionStorage.removeItem(AUTH_TOKEN_KEY);
  window.sessionStorage.removeItem(REFRESH_TOKEN_KEY);
}

export function createNotaryH5TokenManager(): AuthTokenManager {
  return createTokenManager(readInitialTokens(), {
    onTokenSet: (tokens: AuthTokens) => {
      persistTokens(tokens);
      refreshAuthenticatedNotaryH5SdkClients();
    },
    onTokenCleared: () => {
      clearPersistedTokens();
      resetAuthenticatedNotaryH5SdkClients();
    },
  });
}

export function setTokenManager(manager: AuthTokenManager): void {
  tokenManager = manager;
}

export function getTokenManager(): AuthTokenManager | null {
  return tokenManager;
}
