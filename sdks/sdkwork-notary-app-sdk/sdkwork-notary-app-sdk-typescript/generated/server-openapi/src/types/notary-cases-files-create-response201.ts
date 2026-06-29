import type { NotaryDocument } from './notary-document';

export interface NotaryCasesFilesCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
