import type { NotaryCase } from './notary-case';

export interface NotaryCasesCreateResponse201 {
  code: 0;
  data: unknown & { item: NotaryCase; };
  /** Server-owned request correlation id. */
  traceId: string;
}
