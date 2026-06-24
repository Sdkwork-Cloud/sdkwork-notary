import {
  clearNotaryPcIamRuntimeSession,
  getNotaryPcIamRuntime,
  resetNotaryPcIamRuntime,
} from './appAuthRuntime';
import {
  applyNotaryPcSessionTokens,
  isNotaryPcSessionAuthenticated,
  readNotaryPcSessionTokens,
  type NotaryPcSession,
} from './session';
import { resetAuthenticatedNotaryPcSdkClients } from './sdkSessionLifecycle';

export interface NotaryPcAuthService {
  getCurrentSession(): Promise<NotaryPcSession | null>;
  logout(): Promise<void>;
}

interface RuntimeSessionPayload {
  accessToken?: string;
  authToken?: string;
  context?: unknown;
  expiresAt?: number | string;
  refreshToken?: string;
  sessionId?: string;
  user?: NotaryPcSession['user'];
  userInfo?: NotaryPcSession['user'];
}

function toSession(data: RuntimeSessionPayload): NotaryPcSession {
  return {
    accessToken: typeof data.accessToken === 'string' ? data.accessToken : undefined,
    authToken: typeof data.authToken === 'string' ? data.authToken : undefined,
    refreshToken: typeof data.refreshToken === 'string' ? data.refreshToken : undefined,
    sessionId: typeof data.sessionId === 'string' ? data.sessionId : undefined,
    expiresAt: typeof data.expiresAt === 'number'
      ? data.expiresAt
      : typeof data.expiresAt === 'string' && data.expiresAt.trim()
        ? Number(data.expiresAt)
        : undefined,
    context: data.context as NotaryPcSession['context'],
    user: data.user ?? data.userInfo,
  };
}

export const notaryPcAuthService: NotaryPcAuthService = {
  async getCurrentSession() {
    const storedSession = readNotaryPcSessionTokens();
    if (!isNotaryPcSessionAuthenticated(storedSession)) {
      clearNotaryPcIamRuntimeSession();
      return null;
    }

    try {
      const session = await getNotaryPcIamRuntime().service.auth.sessions.current.retrieve();
      return applyNotaryPcSessionTokens(toSession(session as RuntimeSessionPayload));
    } catch {
      clearNotaryPcIamRuntimeSession();
      resetNotaryPcIamRuntime();
      return null;
    }
  },

  async logout() {
    try {
      await getNotaryPcIamRuntime().service.auth.sessions.current.delete();
    } finally {
      clearNotaryPcIamRuntimeSession();
      resetAuthenticatedNotaryPcSdkClients();
      resetNotaryPcIamRuntime();
    }
  },
};
