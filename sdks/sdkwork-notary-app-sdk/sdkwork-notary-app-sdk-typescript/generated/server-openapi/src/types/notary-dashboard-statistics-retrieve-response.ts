import type { NotaryStatistics } from './notary-statistics';

export interface NotaryDashboardStatisticsRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
