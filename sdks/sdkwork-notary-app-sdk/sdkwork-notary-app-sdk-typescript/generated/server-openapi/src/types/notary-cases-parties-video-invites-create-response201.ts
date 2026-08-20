import type { PartyVideoInvite } from './party-video-invite';

export interface NotaryCasesPartiesVideoInvitesCreateResponse201 {
  code: 0;
  data: unknown & { item: PartyVideoInvite; };
  /** Server-owned request correlation id. */
  traceId: string;
}
