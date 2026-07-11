import {
  createNotaryPcTokenManager,
  enableNotaryPcSessionLifecycle,
  refreshAuthenticatedNotaryPcSdkClients,
  registerNotaryPcServiceReset,
  resetAuthenticatedNotaryPcSdkClients,
  setTokenManager,
} from '@sdkwork/notary-pc-core';
import { resetNotaryService } from '@sdkwork/notary-pc-notary';

let iamRuntimeInitialized = false;

export function createIamRuntime(): void {
  if (iamRuntimeInitialized) {
    return;
  }

  const manager = createNotaryPcTokenManager({
    onSessionRefresh: refreshAuthenticatedNotaryPcSdkClients,
    onSessionReset: resetAuthenticatedNotaryPcSdkClients,
  });
  setTokenManager(manager);
  registerNotaryPcServiceReset(resetNotaryService);
  iamRuntimeInitialized = true;
}

export function finalizeIamRuntime(): void {
  enableNotaryPcSessionLifecycle();
}
