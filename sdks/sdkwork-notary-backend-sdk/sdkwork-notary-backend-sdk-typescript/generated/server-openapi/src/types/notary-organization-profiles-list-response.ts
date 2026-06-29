import type { NotaryOrganizationProfile } from './notary-organization-profile';
import type { PageInfo } from './page-info';

export interface NotaryOrganizationProfilesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
