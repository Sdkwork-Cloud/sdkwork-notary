import type { IamAppContext } from '@sdkwork/iam-contracts';
import {
  createTokenManager,
  type AuthTokenManager,
  type AuthTokens,
} from '@sdkwork/sdk-common';

export interface NotaryPcSessionUser {
  avatar?: string;
  displayName?: string;
  email?: string;
  id?: string | number;
  name?: string;
  phone?: string;
  userId?: string;
  username?: string;
}

export interface NotaryPcSessionTokens {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
}

export interface NotaryPcSession extends NotaryPcSessionTokens {
  context?: IamAppContext;
  expiresAt?: number;
  sessionId?: string;
  user?: NotaryPcSessionUser;
}

export interface NotaryPcSessionChangedDetail {
  session: NotaryPcSession | null;
}

const ACCESS_TOKEN_KEY = 'sdkwork.accessToken';
const AUTH_TOKEN_KEY = 'sdkwork.authToken';
const NOTARY_PC_SESSION_KEY = 'sdkwork-notary-pc:session:v1';
export const NOTARY_PC_SESSION_CHANGED_EVENT = 'sdkwork-notary-pc:auth-session-changed';

let notaryPcGlobalTokenManager: AuthTokenManager | null = null;

export interface NotaryPcTokenManagerOptions {
  onSessionRefresh?: () => void;
  onSessionReset?: () => void;
}

function readDevBootstrapAccessToken(): string | undefined {
  const nodeProcess = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process;
  const value = (nodeProcess?.env?.SDKWORK_ACCESS_TOKEN ?? '').trim();
  return value.length > 0 ? value : undefined;
}

function readPersistedSessionRawValue(): string | null {
  if (typeof window === 'undefined') {
    return null;
  }

  return window.sessionStorage.getItem(NOTARY_PC_SESSION_KEY);
}

function writePersistedSessionRawValue(value: string | null): void {
  if (typeof window === 'undefined') {
    return;
  }

  if (value) {
    window.sessionStorage.setItem(NOTARY_PC_SESSION_KEY, value);
  } else {
    window.sessionStorage.removeItem(NOTARY_PC_SESSION_KEY);
  }
}

function readPersistedTokens(): AuthTokens | undefined {
  const raw = readPersistedSessionRawValue();
  if (raw) {
    try {
      const parsed = JSON.parse(raw) as NotaryPcSession;
      if (parsed.accessToken || parsed.authToken) {
        return {
          accessToken: parsed.accessToken,
          authToken: parsed.authToken,
        };
      }
    } catch {
      // Fall through to legacy token keys.
    }
  }

  if (typeof window === 'undefined') {
    return undefined;
  }

  const accessToken = window.sessionStorage.getItem(ACCESS_TOKEN_KEY) ?? undefined;
  const authToken = window.sessionStorage.getItem(AUTH_TOKEN_KEY) ?? undefined;
  if (!accessToken && !authToken) {
    return undefined;
  }

  return { accessToken, authToken };
}

function readInitialTokens(): AuthTokens | undefined {
  const devAccessToken = readDevBootstrapAccessToken();
  if (devAccessToken) {
    return { accessToken: devAccessToken };
  }

  return readPersistedTokens();
}

function persistTokens(tokens: AuthTokens): void {
  if (typeof window === 'undefined') {
    return;
  }

  if (tokens.accessToken) {
    window.sessionStorage.setItem(ACCESS_TOKEN_KEY, tokens.accessToken);
  } else {
    window.sessionStorage.removeItem(ACCESS_TOKEN_KEY);
  }

  if (tokens.authToken) {
    window.sessionStorage.setItem(AUTH_TOKEN_KEY, tokens.authToken);
  } else {
    window.sessionStorage.removeItem(AUTH_TOKEN_KEY);
  }
}

function clearPersistedTokens(): void {
  if (typeof window === 'undefined') {
    return;
  }

  window.sessionStorage.removeItem(ACCESS_TOKEN_KEY);
  window.sessionStorage.removeItem(AUTH_TOKEN_KEY);
  window.sessionStorage.removeItem(NOTARY_PC_SESSION_KEY);
}

function emitSessionChanged(session: NotaryPcSession | null): void {
  if (typeof window === 'undefined') {
    return;
  }

  window.dispatchEvent(new CustomEvent<NotaryPcSessionChangedDetail>(NOTARY_PC_SESSION_CHANGED_EVENT, {
    detail: { session },
  }));
}

export function isNotaryPcSessionAuthenticated(session: NotaryPcSession | null | undefined): boolean {
  return Boolean(session?.accessToken?.trim() || session?.authToken?.trim());
}

export function readNotaryPcSessionTokens(): NotaryPcSession | null {
  const raw = readPersistedSessionRawValue();
  if (!raw) {
    const tokens = readPersistedTokens();
    return tokens ? { ...tokens } : null;
  }

  try {
    const parsed = JSON.parse(raw) as NotaryPcSession;
    return isNotaryPcSessionAuthenticated(parsed) ? parsed : null;
  } catch {
    return null;
  }
}

export function applyNotaryPcSessionTokens(session: NotaryPcSession | null): NotaryPcSession | null {
  const normalized = session && isNotaryPcSessionAuthenticated(session) ? session : null;
  if (normalized) {
    writePersistedSessionRawValue(JSON.stringify(normalized));
    persistTokens(normalized);
    getNotaryPcGlobalTokenManager().setTokens({
      accessToken: normalized.accessToken,
      authToken: normalized.authToken,
    });
  } else {
    writePersistedSessionRawValue(null);
    clearPersistedTokens();
    getNotaryPcGlobalTokenManager().clearTokens();
  }

  emitSessionChanged(normalized);
  return normalized;
}

export function clearNotaryPcSessionTokens(): void {
  applyNotaryPcSessionTokens(null);
}

export function createNotaryPcTokenManager(
  options: NotaryPcTokenManagerOptions = {},
): AuthTokenManager {
  const manager = createTokenManager(readInitialTokens(), {
    onTokenSet: (tokens: AuthTokens) => {
      persistTokens(tokens);
      options.onSessionRefresh?.();
    },
    onTokenCleared: () => {
      clearPersistedTokens();
      options.onSessionReset?.();
    },
  });
  setNotaryPcGlobalTokenManager(manager);
  return manager;
}

export function setNotaryPcGlobalTokenManager(manager: AuthTokenManager): void {
  notaryPcGlobalTokenManager = manager;
}

export function getNotaryPcGlobalTokenManager(): AuthTokenManager {
  if (!notaryPcGlobalTokenManager) {
    notaryPcGlobalTokenManager = createNotaryPcTokenManager();
  }
  return notaryPcGlobalTokenManager;
}

// Backward-compatible aliases for app bootstrap.
export const createNotaryPcTokenManagerFromBootstrap = createNotaryPcTokenManager;
export const getTokenManager = getNotaryPcGlobalTokenManager;
export const setTokenManager = setNotaryPcGlobalTokenManager;
