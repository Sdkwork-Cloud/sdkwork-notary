import type { NotaryCase } from './notary-case';

export interface NotaryCasesRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
