import { notaryPcAccessService } from './notaryPcAccessService';
import { resetNotaryPcSdkClients } from './notaryAppSdk';

let sessionLifecycleEnabled = false;
let resetNotaryServiceDelegate: (() => void) | null = null;

export function enableNotaryPcSessionLifecycle(): void {
  sessionLifecycleEnabled = true;
}

export function registerNotaryPcServiceReset(handler: () => void): void {
  resetNotaryServiceDelegate = handler;
}

export function resetAuthenticatedNotaryPcSdkClients(): void {
  if (!sessionLifecycleEnabled) {
    return;
  }

  resetNotaryPcSdkClients();
  notaryPcAccessService.reset();
  resetNotaryServiceDelegate?.();
}

export type RefreshNotaryPcSdkClients = () => void;

let refreshNotaryPcSdkClients: RefreshNotaryPcSdkClients | null = null;

export function registerNotaryPcSdkClientRefresh(handler: RefreshNotaryPcSdkClients): void {
  refreshNotaryPcSdkClients = handler;
}

export function refreshAuthenticatedNotaryPcSdkClients(): void {
  if (!sessionLifecycleEnabled) {
    return;
  }

  resetAuthenticatedNotaryPcSdkClients();
  refreshNotaryPcSdkClients?.();
}
