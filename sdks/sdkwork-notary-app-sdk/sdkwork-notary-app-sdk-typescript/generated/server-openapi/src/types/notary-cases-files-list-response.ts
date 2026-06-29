import type { NotaryDocument } from './notary-document';
import type { PageInfo } from './page-info';

export interface NotaryCasesFilesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
