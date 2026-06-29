import type { NotaryMatter } from './notary-matter';
import type { PageInfo } from './page-info';

export interface NotaryMattersListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
