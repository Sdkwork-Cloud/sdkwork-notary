import type { PageInfo } from './page-info';
import type { Party } from './party';

export interface NotaryCasesPartiesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
