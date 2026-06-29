import type { NotaryMatter } from './notary-matter';

export interface NotaryMattersCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
