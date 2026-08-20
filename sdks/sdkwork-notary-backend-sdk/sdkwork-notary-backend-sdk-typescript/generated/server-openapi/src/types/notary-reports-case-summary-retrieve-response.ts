import type { NotaryCaseSummary } from './notary-case-summary';

export interface NotaryReportsCaseSummaryRetrieveResponse {
  code: 0;
  data: unknown & { item: NotaryCaseSummary; };
  /** Server-owned request correlation id. */
  traceId: string;
}
