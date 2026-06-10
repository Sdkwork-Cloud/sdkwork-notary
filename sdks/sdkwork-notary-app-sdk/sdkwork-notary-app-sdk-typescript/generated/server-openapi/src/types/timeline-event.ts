export interface TimelineEvent {
  id?: string;
  time: string;
  event: string;
  actor: string;
  eventType?: string;
  detail?: string;
}
