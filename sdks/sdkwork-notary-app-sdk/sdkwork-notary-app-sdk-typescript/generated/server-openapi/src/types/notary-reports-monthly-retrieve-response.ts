import type { MonthlyReport } from './monthly-report';

export interface NotaryReportsMonthlyRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
