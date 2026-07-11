import type { NotaryCase } from './notary-case';

export interface NotaryCasesUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
