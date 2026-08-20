import type { Party } from './party';

export interface NotaryCasesPartiesCreateResponse201 {
  code: 0;
  data: unknown & { item: Party; };
  /** Server-owned request correlation id. */
  traceId: string;
}
