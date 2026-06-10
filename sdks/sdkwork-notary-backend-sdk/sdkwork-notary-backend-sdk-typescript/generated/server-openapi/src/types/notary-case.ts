import type { NotaryCaseStatus } from './notary-case-status';

export interface NotaryCase {
  id: string;
  caseNo: string;
  tenantId?: string;
  organizationId?: string;
  title: string;
  status: NotaryCaseStatus;
  applicantUserId?: string;
  applicantName?: string;
  primaryNotaryMembershipId?: string;
  primaryNotaryName?: string;
  orderId: string;
  orderItemId: string;
  skuId: string;
  matterTitle?: string;
  feeAmount?: string;
  currencyCode?: string;
  driveSpaceId: string;
  driveSpaceType: 'notary';
  driveFolderNodeId: string;
  chainHash?: string;
  submittedAt?: string;
  completedAt?: string;
  version?: string;
}
