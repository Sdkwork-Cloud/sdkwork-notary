import type { NotaryStaffMember } from './notary-staff-member';
import type { PageInfo } from './page-info';

export interface NotaryStaffMemberPage {
  items: NotaryStaffMember[];
  pageInfo: PageInfo;
}
