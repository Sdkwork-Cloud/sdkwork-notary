import type { NotaryStaffMember } from './notary-staff-member';
import type { PageInfo } from './page-info';

export interface NotaryStaffListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
