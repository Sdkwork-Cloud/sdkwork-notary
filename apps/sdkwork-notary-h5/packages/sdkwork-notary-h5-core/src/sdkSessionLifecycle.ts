import { resetNotaryH5SdkClients } from './notaryAppSdk';

let sessionLifecycleEnabled = false;

export function enableNotaryH5SessionLifecycle(): void {
  sessionLifecycleEnabled = true;
}

export function resetAuthenticatedNotaryH5SdkClients(): void {
  if (!sessionLifecycleEnabled) {
    return;
  }

  resetNotaryH5SdkClients();
}

export type RefreshNotaryH5SdkClients = () => void;

let refreshNotaryH5SdkClients: RefreshNotaryH5SdkClients | null = null;

export function registerNotaryH5SdkClientRefresh(handler: RefreshNotaryH5SdkClients): void {
  refreshNotaryH5SdkClients = handler;
}

export function refreshAuthenticatedNotaryH5SdkClients(): void {
  if (!sessionLifecycleEnabled) {
    return;
  }

  resetAuthenticatedNotaryH5SdkClients();
  refreshNotaryH5SdkClients?.();
}
