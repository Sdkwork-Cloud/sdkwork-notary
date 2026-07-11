import type { NotaryCasePage } from './notary-case-page';

export interface NotaryCasesListResponse {
  code: 0;
  data: unknown & NotaryCasePage;
  /** Server-owned request correlation id. */
  traceId: string;
}
