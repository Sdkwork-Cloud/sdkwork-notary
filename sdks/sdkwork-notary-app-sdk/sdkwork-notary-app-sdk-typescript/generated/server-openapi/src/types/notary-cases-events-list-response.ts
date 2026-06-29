import type { PageInfo } from './page-info';
import type { TimelineEvent } from './timeline-event';

export interface NotaryCasesEventsListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
