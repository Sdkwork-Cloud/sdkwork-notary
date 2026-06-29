import type { NotaryAccess } from './notary-access';

export interface NotaryAccessRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
