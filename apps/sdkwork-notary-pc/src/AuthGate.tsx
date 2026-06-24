import { useEffect, useMemo, useState, type ReactNode } from 'react';
import { Navigate, useLocation, useNavigate } from 'react-router-dom';
import { SdkworkIamAuthRoutes } from '@sdkwork/auth-pc-react';
import { LoadingState } from '@sdkwork/notary-pc-commons';
import {
  getNotaryPcIamRuntime,
  isNotaryPcSessionAuthenticated,
  NOTARY_PC_SESSION_CHANGED_EVENT,
  notaryPcAuthService,
  readNotaryPcSessionTokens,
  resolveNotaryPcAuthAppearance,
  resolveNotaryPcAuthRuntimeConfig,
  type NotaryPcSession,
  type NotaryPcSessionChangedDetail,
} from '@sdkwork/notary-pc-core';

export interface AuthGateProps {
  children: ReactNode;
}

const AUTH_BASE_PATH = '/auth';

function isAuthRoute(pathname: string): boolean {
  return pathname === AUTH_BASE_PATH || pathname.startsWith(`${AUTH_BASE_PATH}/`);
}

function resolveRedirectTarget(pathname: string, search: string, hash: string): string {
  const target = `${pathname}${search}${hash}`;
  if (isAuthRoute(pathname)) {
    return '/notary';
  }

  return target || '/notary';
}

function buildAuthLoginPath(redirectTarget: string): string {
  const params = new URLSearchParams();
  params.set('redirect', redirectTarget || '/notary');
  return `${AUTH_BASE_PATH}/login?${params.toString()}`;
}

function resolveAuthLocale(): string | null {
  if (typeof navigator === 'undefined') {
    return null;
  }

  const language = navigator.language.trim();
  return language || null;
}

export function AuthGate({ children }: AuthGateProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const [isBootstrapped, setIsBootstrapped] = useState(false);
  const [session, setSession] = useState<NotaryPcSession | null>(null);

  const redirectTarget = useMemo(
    () => resolveRedirectTarget(location.pathname, location.search, location.hash),
    [location.hash, location.pathname, location.search],
  );
  const isAuthenticated = isNotaryPcSessionAuthenticated(session)
    && isNotaryPcSessionAuthenticated(readNotaryPcSessionTokens());
  const isAuthPath = isAuthRoute(location.pathname);

  useEffect(() => {
    let disposed = false;

    const bootstrapAuthSession = async () => {
      const storedSession = readNotaryPcSessionTokens();
      if (!isNotaryPcSessionAuthenticated(storedSession)) {
        setSession(null);
        setIsBootstrapped(true);
        return;
      }

      setIsBootstrapped(false);

      try {
        const nextSession = await notaryPcAuthService.getCurrentSession();
        if (!disposed) {
          setSession(nextSession);
          setIsBootstrapped(true);
        }
      } catch {
        if (!disposed) {
          setSession(null);
          setIsBootstrapped(true);
        }
      }
    };

    void bootstrapAuthSession();

    return () => {
      disposed = true;
    };
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') {
      return undefined;
    }

    const handleSessionChanged = (event: Event) => {
      const detail = (event as CustomEvent<NotaryPcSessionChangedDetail>).detail;
      setSession(detail?.session ?? readNotaryPcSessionTokens());
      setIsBootstrapped(true);
    };

    window.addEventListener(NOTARY_PC_SESSION_CHANGED_EVENT, handleSessionChanged);
    return () => {
      window.removeEventListener(NOTARY_PC_SESSION_CHANGED_EVENT, handleSessionChanged);
    };
  }, []);

  useEffect(() => {
    if (!isBootstrapped || isAuthenticated || isAuthPath) {
      return;
    }

    navigate(buildAuthLoginPath(redirectTarget), { replace: true });
  }, [isAuthPath, isAuthenticated, isBootstrapped, navigate, redirectTarget]);

  if (!isBootstrapped) {
    return (
      <main className="notary-pc-auth-gate">
        <LoadingState label="Loading authentication…" />
      </main>
    );
  }

  if (isAuthenticated && isAuthPath) {
    return <Navigate replace to={redirectTarget} />;
  }

  if (isAuthenticated) {
    return <>{children}</>;
  }

  return (
    <main className="notary-pc-auth-gate">
      <SdkworkIamAuthRoutes
        appearance={resolveNotaryPcAuthAppearance()}
        basePath={AUTH_BASE_PATH}
        getRuntime={getNotaryPcIamRuntime}
        homePath="/notary"
        locale={resolveAuthLocale()}
        runtimeConfig={resolveNotaryPcAuthRuntimeConfig()}
        viewportMode="fixed"
      />
    </main>
  );
}
