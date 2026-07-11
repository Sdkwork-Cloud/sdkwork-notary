import { useEffect, useMemo, useState } from 'react';
import {
  NotaryPcAdminRuntimeProvider,
  type NotaryPcAdminOperator,
} from '@sdkwork/notary-pc-admin-core';
import { NotaryPcAdminRoutes } from '@sdkwork/notary-pc-admin-shell';
import {
  NOTARY_PC_SESSION_CHANGED_EVENT,
  readNotaryPcSessionTokens,
  type NotaryPcSession,
  type NotaryPcSessionChangedDetail,
} from '@sdkwork/notary-pc-core';

import { resolveEnvironment } from './bootstrap/environment';

interface AdminSessionContext {
  organizationId?: unknown;
  permissionScope?: unknown;
  tenantId?: unknown;
  userId?: unknown;
}

function optionalString(value: unknown): string | undefined {
  if (typeof value !== 'string') {
    return undefined;
  }
  const normalized = value.trim();
  return normalized || undefined;
}

function permissionScope(value: unknown): readonly string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value
    .filter((permission): permission is string => typeof permission === 'string')
    .map((permission) => permission.trim())
    .filter(Boolean);
}

function sessionUserId(session: NotaryPcSession | null): string | null {
  const context = session?.context as AdminSessionContext | undefined;
  const candidate = context?.userId
    ?? session?.user?.userId
    ?? session?.user?.id;
  if (candidate === undefined || candidate === null) {
    return null;
  }
  const value = String(candidate).trim();
  return value || null;
}

function sessionDisplayName(session: NotaryPcSession, operatorId: string): string {
  return session.user?.displayName?.trim()
    || session.user?.name?.trim()
    || session.user?.username?.trim()
    || operatorId;
}

function resolveOperator(session: NotaryPcSession | null): NotaryPcAdminOperator | null {
  const operatorId = sessionUserId(session);
  if (!session || !operatorId) {
    return null;
  }

  const context = session.context as AdminSessionContext | undefined;
  return {
    operatorId,
    displayName: sessionDisplayName(session, operatorId),
    permissions: permissionScope(context?.permissionScope),
    tenantId: optionalString(context?.tenantId),
    organizationId: optionalString(context?.organizationId),
  };
}

export function AdminSurface() {
  const [session, setSession] = useState<NotaryPcSession | null>(
    () => readNotaryPcSessionTokens(),
  );
  const operator = useMemo(() => resolveOperator(session), [session]);
  const backendConfig = useMemo(
    () => ({ baseUrl: resolveEnvironment().backendApiBaseUrl, platform: 'pc' as const }),
    [],
  );

  useEffect(() => {
    const handleSessionChanged = (event: Event) => {
      const detail = (event as CustomEvent<NotaryPcSessionChangedDetail>).detail;
      setSession(detail?.session ?? readNotaryPcSessionTokens());
    };

    window.addEventListener(NOTARY_PC_SESSION_CHANGED_EVENT, handleSessionChanged);
    return () => window.removeEventListener(NOTARY_PC_SESSION_CHANGED_EVENT, handleSessionChanged);
  }, []);

  if (!operator) {
    return (
      <main className="notary-pc-admin-context-error" role="alert">
        Administrator identity context is unavailable.
      </main>
    );
  }

  return (
    <NotaryPcAdminRuntimeProvider backendConfig={backendConfig} operator={operator}>
      <NotaryPcAdminRoutes />
    </NotaryPcAdminRuntimeProvider>
  );
}
