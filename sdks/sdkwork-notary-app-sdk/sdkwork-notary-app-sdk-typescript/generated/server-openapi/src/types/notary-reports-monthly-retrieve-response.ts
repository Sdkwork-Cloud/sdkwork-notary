import type { MonthlyReport } from './monthly-report';

export interface NotaryReportsMonthlyRetrieveResponse {
  code: 0;
  data: unknown & { item: MonthlyReport; };
  /** Server-owned request correlation id. */
  traceId: string;
}
