import {
  createContext,
  useContext,
  useMemo,
  type ReactNode,
} from 'react';
import type { SdkworkNotaryBackendClient } from '@sdkwork/notary-backend-sdk';

import {
  createNotaryPcAdminBackendSdkClient,
  type NotaryPcAdminBackendSdkConfig,
} from './sdk/backendSdk';
import {
  normalizeNotaryPcAdminOperator,
  type NotaryPcAdminOperator,
} from './operator';

interface NotaryPcAdminRuntimeContextValue {
  backendClient: SdkworkNotaryBackendClient;
  operator: NotaryPcAdminOperator;
}

export interface NotaryPcAdminRuntimeProviderProps {
  children: ReactNode;
  operator: NotaryPcAdminOperator;
  backendClient?: SdkworkNotaryBackendClient;
  backendConfig?: NotaryPcAdminBackendSdkConfig;
}

const NotaryPcAdminRuntimeContext = createContext<NotaryPcAdminRuntimeContextValue | null>(null);

function resolveBackendClient(
  client: SdkworkNotaryBackendClient | undefined,
  config: NotaryPcAdminBackendSdkConfig | undefined,
): SdkworkNotaryBackendClient {
  if (client) {
    return client;
  }
  if (!config) {
    throw new Error('NotaryPcAdminRuntimeProvider requires backendClient or backendConfig.');
  }
  return createNotaryPcAdminBackendSdkClient(config);
}

export function NotaryPcAdminRuntimeProvider({
  backendClient,
  backendConfig,
  children,
  operator,
}: NotaryPcAdminRuntimeProviderProps) {
  const value = useMemo<NotaryPcAdminRuntimeContextValue>(
    () => ({
      backendClient: resolveBackendClient(backendClient, backendConfig),
      operator: normalizeNotaryPcAdminOperator(operator),
    }),
    [backendClient, backendConfig, operator],
  );

  return (
    <NotaryPcAdminRuntimeContext.Provider value={value}>
      {children}
    </NotaryPcAdminRuntimeContext.Provider>
  );
}

function useNotaryPcAdminRuntime(): NotaryPcAdminRuntimeContextValue {
  const runtime = useContext(NotaryPcAdminRuntimeContext);
  if (!runtime) {
    throw new Error('Notary PC admin runtime is unavailable. Mount NotaryPcAdminRuntimeProvider.');
  }
  return runtime;
}

export function useNotaryPcAdminBackendSdkClient(): SdkworkNotaryBackendClient {
  return useNotaryPcAdminRuntime().backendClient;
}

export function useNotaryPcAdminOperator(): NotaryPcAdminOperator {
  return useNotaryPcAdminRuntime().operator;
}
