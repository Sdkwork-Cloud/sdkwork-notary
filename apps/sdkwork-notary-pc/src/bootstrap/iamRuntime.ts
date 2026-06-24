import {
  createNotaryPcTokenManager,
  enableNotaryPcSessionLifecycle,
  getTokenManager,
  refreshAuthenticatedNotaryPcSdkClients,
  registerNotaryPcServiceReset,
  resetAuthenticatedNotaryPcSdkClients,
  setTokenManager,
} from '@sdkwork/notary-pc-core';
import { resetNotaryService } from '@sdkwork/notary-pc-notary';

export function createIamRuntime(): void {
  if (getTokenManager()) {
    return;
  }

  const manager = createNotaryPcTokenManager({
    onSessionRefresh: refreshAuthenticatedNotaryPcSdkClients,
    onSessionReset: resetAuthenticatedNotaryPcSdkClients,
  });
  setTokenManager(manager);
  registerNotaryPcServiceReset(resetNotaryService);
}

export function finalizeIamRuntime(): void {
  enableNotaryPcSessionLifecycle();
}
