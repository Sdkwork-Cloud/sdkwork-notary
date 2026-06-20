import { createTokenManager, getTokenManager, setTokenManager } from './tokenManager';

export function createIamRuntime(): void {
  if (!getTokenManager()) {
    setTokenManager(
      createTokenManager(() => {
        if (typeof window === 'undefined') {
          return undefined;
        }
        return window.sessionStorage.getItem('sdkwork.accessToken') ?? undefined;
      }),
    );
  }
}
