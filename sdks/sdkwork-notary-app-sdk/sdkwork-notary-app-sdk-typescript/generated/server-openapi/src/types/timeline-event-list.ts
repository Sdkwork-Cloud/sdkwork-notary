import type { PageInfo } from './page-info';
import type { TimelineEvent } from './timeline-event';

export interface TimelineEventList {
  items: TimelineEvent[];
  pageInfo: PageInfo;
}
