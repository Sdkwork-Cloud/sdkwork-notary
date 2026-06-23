import { createNotaryPcTokenManager, getTokenManager, setTokenManager } from './tokenManager';

export function createIamRuntime(): void {
  if (!getTokenManager()) {
    setTokenManager(createNotaryPcTokenManager());
  }
}
