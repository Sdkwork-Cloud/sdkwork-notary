import type { ReactNode } from 'react';
import { useEffect } from 'react';

import { LoadingState } from '@sdkwork/notary-pc-commons';

import { resolveEnvironment } from './bootstrap/environment';
import { getTokenManager } from './bootstrap/tokenManager';

export interface AuthGateProps {
  children: ReactNode;
}

function resolveLoginRedirectUrl(): string {
  const gateway = import.meta.env.VITE_SDKWORK_NOTARY_PLATFORM_API_GATEWAY_HTTP_URL;
  const baseUrl = typeof gateway === 'string' && gateway.trim()
    ? gateway.trim()
    : resolveEnvironment().apiBaseUrl;
  const returnUrl = encodeURIComponent(window.location.href);
  return `${baseUrl.replace(/\/$/, '')}/auth/login?redirect=${returnUrl}`;
}

/**
 * Route guard for authenticated PC sessions.
 * Production redirects to platform IAM login; appbase PC login UI mounts here when available.
 */
export function AuthGate({ children }: AuthGateProps) {
  const tokenManager = getTokenManager();
  const hasSession = Boolean(
    tokenManager?.hasAccessToken?.() || tokenManager?.hasAuthToken?.(),
  );

  useEffect(() => {
    if (!hasSession && import.meta.env.PROD) {
      window.location.replace(resolveLoginRedirectUrl());
    }
  }, [hasSession]);

  if (!hasSession && import.meta.env.PROD) {
    return (
      <main className="notary-pc-auth-gate">
        <LoadingState label="Redirecting to sign in…" />
      </main>
    );
  }

  return <>{children}</>;
}
