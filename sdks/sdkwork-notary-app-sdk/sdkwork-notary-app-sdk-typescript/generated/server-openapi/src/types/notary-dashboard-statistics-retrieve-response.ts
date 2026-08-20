import type { NotaryStatistics } from './notary-statistics';

export interface NotaryDashboardStatisticsRetrieveResponse {
  code: 0;
  data: unknown & { item: NotaryStatistics; };
  /** Server-owned request correlation id. */
  traceId: string;
}
