import type { AuthTokenManager } from '@sdkwork/sdk-common';

let tokenManager: AuthTokenManager | null = null;

export function createTokenManager(readAccessToken: () => string | undefined): AuthTokenManager {
  return {
    getAccessToken: readAccessToken,
  };
}

export function setTokenManager(manager: AuthTokenManager): void {
  tokenManager = manager;
}

export function getTokenManager(): AuthTokenManager | null {
  return tokenManager;
}
