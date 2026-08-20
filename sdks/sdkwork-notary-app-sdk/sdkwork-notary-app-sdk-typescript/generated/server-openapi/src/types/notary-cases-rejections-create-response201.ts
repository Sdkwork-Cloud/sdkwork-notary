import type { NotaryCase } from './notary-case';

export interface NotaryCasesRejectionsCreateResponse201 {
  code: 0;
  data: unknown & { item: NotaryCase; };
  /** Server-owned request correlation id. */
  traceId: string;
}
