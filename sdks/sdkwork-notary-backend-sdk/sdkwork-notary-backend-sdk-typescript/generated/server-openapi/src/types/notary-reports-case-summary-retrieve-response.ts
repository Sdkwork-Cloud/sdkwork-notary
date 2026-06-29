import type { NotaryCaseSummary } from './notary-case-summary';

export interface NotaryReportsCaseSummaryRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
