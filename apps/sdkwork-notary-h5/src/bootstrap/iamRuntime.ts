import { createNotaryH5TokenManager, getTokenManager, setTokenManager } from './tokenManager';

export function createIamRuntime(): void {
  if (!getTokenManager()) {
    setTokenManager(createNotaryH5TokenManager());
  }
}
