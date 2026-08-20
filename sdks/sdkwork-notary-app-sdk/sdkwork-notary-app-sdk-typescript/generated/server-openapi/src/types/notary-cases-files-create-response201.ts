import type { NotaryDocument } from './notary-document';

export interface NotaryCasesFilesCreateResponse201 {
  code: 0;
  data: unknown & { item: NotaryDocument; };
  /** Server-owned request correlation id. */
  traceId: string;
}
