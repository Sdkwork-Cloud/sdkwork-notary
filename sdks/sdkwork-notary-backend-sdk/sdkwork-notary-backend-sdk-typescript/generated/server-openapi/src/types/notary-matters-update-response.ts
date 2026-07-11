import type { NotaryMatter } from './notary-matter';

export interface NotaryMattersUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
