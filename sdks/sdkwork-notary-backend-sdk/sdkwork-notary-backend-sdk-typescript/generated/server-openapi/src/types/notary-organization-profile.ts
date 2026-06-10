export interface NotaryOrganizationProfile {
  id: string;
  tenantId: string;
  organizationId: string;
  organizationName?: string;
  organizationVerificationStatus?: string;
  status: 'active' | 'suspended' | 'closed';
  driveSpaceId: string;
  driveSpaceType: 'notary';
  openedByMembershipId?: string;
  openedAt: string;
  settings?: Record<string, unknown>;
  version: string;
}
