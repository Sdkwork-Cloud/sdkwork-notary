import type { NotaryDocumentList } from './notary-document-list';

export interface NotaryCasesFilesListResponse {
  code: 0;
  data: unknown & NotaryDocumentList;
  /** Server-owned request correlation id. */
  traceId: string;
}
