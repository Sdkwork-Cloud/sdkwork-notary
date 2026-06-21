import type { ReactNode } from 'react';

import { getTokenManager } from './bootstrap/tokenManager';

export interface AuthGateProps {
  children: ReactNode;
}

/**
 * Route guard for authenticated H5 sessions.
 * Appbase IAM H5 login routes mount here when @sdkwork/appbase H5 auth packages are wired.
 */
export function AuthGate({ children }: AuthGateProps) {
  const tokenManager = getTokenManager();
  const hasSession = Boolean(
    tokenManager?.hasAccessToken?.() || tokenManager?.hasAuthToken?.(),
  );

  if (!hasSession && import.meta.env.PROD) {
    return (
      <main className="notary-h5-auth-gate">
        <h1>SDKWork Notary</h1>
        <p>Sign in to continue.</p>
      </main>
    );
  }

  return <>{children}</>;
}
