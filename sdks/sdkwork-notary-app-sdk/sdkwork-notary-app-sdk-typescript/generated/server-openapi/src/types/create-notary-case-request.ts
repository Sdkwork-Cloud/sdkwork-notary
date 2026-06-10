import type { CreateNotaryPartyRequest } from './create-notary-party-request';

export interface CreateNotaryCaseRequest {
  skuId: string;
  title: string;
  description?: string;
  remarks?: string;
  applicantName: string;
  /** Organization member id selected as the primary notary for this case. */
  primaryNotaryMembershipId?: string;
  parties?: CreateNotaryPartyRequest[];
  initialDriveNodeIds?: string[];
  driveFolderName?: string;
}
