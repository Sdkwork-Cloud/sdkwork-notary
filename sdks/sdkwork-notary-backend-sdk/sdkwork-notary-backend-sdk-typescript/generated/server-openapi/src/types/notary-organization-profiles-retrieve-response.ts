import type { NotaryOrganizationProfile } from './notary-organization-profile';

export interface NotaryOrganizationProfilesRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
