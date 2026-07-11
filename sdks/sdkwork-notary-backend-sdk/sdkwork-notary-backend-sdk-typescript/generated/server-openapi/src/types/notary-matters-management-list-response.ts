import type { NotaryMatterPage } from './notary-matter-page';

export interface NotaryMattersManagementListResponse {
  code: 0;
  data: unknown & NotaryMatterPage;
  /** Server-owned request correlation id. */
  traceId: string;
}
