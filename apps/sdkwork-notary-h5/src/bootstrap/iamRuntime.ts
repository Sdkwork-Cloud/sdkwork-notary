import { createNotaryH5TokenManager, getTokenManager, setTokenManager } from './tokenManager';
import { enableNotaryH5SessionLifecycle } from '@sdkwork/notary-h5-core';

export function createIamRuntime(): void {
  if (getTokenManager()) {
    return;
  }

  setTokenManager(createNotaryH5TokenManager());
}

export function finalizeIamRuntime(): void {
  enableNotaryH5SessionLifecycle();
}
