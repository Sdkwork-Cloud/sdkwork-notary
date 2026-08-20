import type { NotaryMatter } from './notary-matter';

export interface NotaryMattersUpdateResponse {
  code: 0;
  data: unknown & { item: NotaryMatter; };
  /** Server-owned request correlation id. */
  traceId: string;
}
