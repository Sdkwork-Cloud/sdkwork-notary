import type { NotaryPartyList } from './notary-party-list';

export interface NotaryCasesPartiesListResponse {
  code: 0;
  data: unknown & NotaryPartyList;
  /** Server-owned request correlation id. */
  traceId: string;
}
