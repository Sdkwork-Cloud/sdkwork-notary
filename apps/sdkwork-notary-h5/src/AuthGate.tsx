import type { ReactNode } from 'react';
import { useEffect } from 'react';

import { LoadingState } from '@sdkwork/notary-h5-commons';

import { resolveEnvironment } from './bootstrap/environment';
import { getTokenManager } from './bootstrap/tokenManager';

export interface AuthGateProps {
  children: ReactNode;
}

function isAuthRoutePath(pathname: string): boolean {
  return pathname === '/auth' || pathname.startsWith('/auth/');
}

function resolveLoginRedirectUrl(): string {
  const gateway = import.meta.env.VITE_SDKWORK_NOTARY_PLATFORM_API_GATEWAY_HTTP_URL;
  const baseUrl = typeof gateway === 'string' && gateway.trim()
    ? gateway.trim()
    : resolveEnvironment().apiBaseUrl;
  const { pathname, search } = window.location;
  // Never re-wrap an auth-route URL: encoding the whole current URL again
  // nests the `redirect` param one level deeper on every bounce. Reuse the
  // existing return target when already on the auth surface.
  if (isAuthRoutePath(pathname)) {
    const existing = /[?&]redirect=([^&]*)/u.exec(search)?.[1];
    return `${baseUrl.replace(/\/$/, '')}/auth/login${existing ? `?redirect=${existing}` : ''}`;
  }
  const returnUrl = encodeURIComponent(pathname + search);
  return `${baseUrl.replace(/\/$/, '')}/auth/login?redirect=${returnUrl}`;
}

/**
 * Route guard for authenticated H5 sessions.
 * Production redirects to platform IAM login; appbase H5 login UI mounts here when available.
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
      <main className="notary-h5-auth-gate">
        <LoadingState label="Redirecting to sign in…" />
      </main>
    );
  }

  return <>{children}</>;
}
