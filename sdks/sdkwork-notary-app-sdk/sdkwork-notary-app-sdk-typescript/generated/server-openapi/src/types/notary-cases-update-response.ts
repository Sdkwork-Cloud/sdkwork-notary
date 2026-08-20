import type { NotaryCase } from './notary-case';

export interface NotaryCasesUpdateResponse {
  code: 0;
  data: unknown & { item: NotaryCase; };
  /** Server-owned request correlation id. */
  traceId: string;
}
