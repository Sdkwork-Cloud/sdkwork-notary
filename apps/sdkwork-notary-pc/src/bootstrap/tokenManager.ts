import {
  createTokenManager,
  type AuthTokenManager,
  type AuthTokens,
} from '@sdkwork/sdk-common';

const ACCESS_TOKEN_KEY = 'sdkwork.accessToken';
const AUTH_TOKEN_KEY = 'sdkwork.authToken';

let tokenManager: AuthTokenManager | null = null;

function readPersistedTokens(): AuthTokens | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const accessToken = window.sessionStorage.getItem(ACCESS_TOKEN_KEY) ?? undefined;
  const authToken = window.sessionStorage.getItem(AUTH_TOKEN_KEY) ?? undefined;

  if (!accessToken && !authToken) {
    return undefined;
  }

  return { accessToken, authToken };
}

function persistTokens(tokens: AuthTokens): void {
  if (typeof window === 'undefined') {
    return;
  }

  if (tokens.accessToken) {
    window.sessionStorage.setItem(ACCESS_TOKEN_KEY, tokens.accessToken);
  } else {
    window.sessionStorage.removeItem(ACCESS_TOKEN_KEY);
  }

  if (tokens.authToken) {
    window.sessionStorage.setItem(AUTH_TOKEN_KEY, tokens.authToken);
  } else {
    window.sessionStorage.removeItem(AUTH_TOKEN_KEY);
  }
}

function clearPersistedTokens(): void {
  if (typeof window === 'undefined') {
    return;
  }

  window.sessionStorage.removeItem(ACCESS_TOKEN_KEY);
  window.sessionStorage.removeItem(AUTH_TOKEN_KEY);
}

export function createNotaryPcTokenManager(): AuthTokenManager {
  return createTokenManager(readPersistedTokens(), {
    onTokenSet: persistTokens,
    onTokenCleared: clearPersistedTokens,
  });
}

export function setTokenManager(manager: AuthTokenManager): void {
  tokenManager = manager;
}

export function getTokenManager(): AuthTokenManager | null {
  return tokenManager;
}
