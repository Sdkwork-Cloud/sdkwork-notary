import type { NotaryStaffMemberPage } from './notary-staff-member-page';

export interface NotaryStaffListResponse {
  code: 0;
  data: unknown & NotaryStaffMemberPage;
  /** Server-owned request correlation id. */
  traceId: string;
}
