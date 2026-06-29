import type { NotaryCase } from './notary-case';
import type { PageInfo } from './page-info';

export interface NotaryCasesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
