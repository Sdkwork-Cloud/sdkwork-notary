import type { TimelineEventList } from './timeline-event-list';

export interface NotaryCasesEventsListResponse {
  code: 0;
  data: unknown & TimelineEventList;
  /** Server-owned request correlation id. */
  traceId: string;
}
