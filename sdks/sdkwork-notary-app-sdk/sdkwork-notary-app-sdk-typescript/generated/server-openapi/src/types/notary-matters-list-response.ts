import type { NotaryMatterPage } from './notary-matter-page';

export interface NotaryMattersListResponse {
  code: 0;
  data: unknown & NotaryMatterPage;
  /** Server-owned request correlation id. */
  traceId: string;
}
