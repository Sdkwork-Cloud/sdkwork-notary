import type { NotaryCaseStatus } from './notary-case-status';
import type { NotaryDocument } from './notary-document';
import type { Party } from './party';
import type { TimelineEvent } from './timeline-event';

export interface NotaryCase {
  id: string;
  caseNo: string;
  createTime: string;
  processTime?: string;
  applicant: string;
  applicantUserId?: string;
  title: string;
  notary: string;
  primaryNotaryMembershipId?: string;
  remarks: string;
  type: string;
  status: NotaryCaseStatus;
  fee: string;
  currencyCode?: string;
  hash?: string;
  orderId: string;
  orderItemId: string;
  skuId: string;
  driveSpaceId: string;
  driveSpaceType: 'notary';
  driveFolderNodeId: string;
  parties: Party[];
  documents: NotaryDocument[];
  timeline: TimelineEvent[];
  version?: string;
}
