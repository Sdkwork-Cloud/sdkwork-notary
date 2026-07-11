import type { NotaryOrganizationProfilePage } from './notary-organization-profile-page';

export interface NotaryOrganizationProfilesListResponse {
  code: 0;
  data: unknown & NotaryOrganizationProfilePage;
  /** Server-owned request correlation id. */
  traceId: string;
}
