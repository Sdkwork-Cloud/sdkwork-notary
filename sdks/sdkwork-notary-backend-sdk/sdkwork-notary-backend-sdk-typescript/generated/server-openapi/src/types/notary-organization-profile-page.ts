import type { NotaryOrganizationProfile } from './notary-organization-profile';
import type { PageInfo } from './page-info';

export interface NotaryOrganizationProfilePage {
  items: NotaryOrganizationProfile[];
  pageInfo: PageInfo;
}
