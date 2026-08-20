import type { NotaryOrganizationProfile } from './notary-organization-profile';

export interface NotaryOrganizationProfilesRetrieveResponse {
  code: 0;
  data: unknown & { item: NotaryOrganizationProfile; };
  /** Server-owned request correlation id. */
  traceId: string;
}
