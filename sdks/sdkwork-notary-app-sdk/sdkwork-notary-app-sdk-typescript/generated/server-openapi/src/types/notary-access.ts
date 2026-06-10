export interface NotaryAccess {
  visible: boolean;
  organizationVerified: boolean;
  notaryBusinessEnabled: boolean;
  tenantId?: string;
  organizationId?: string;
  memberId: string;
  roles?: string[];
  positions?: string[];
  departments?: string[];
  permissions: string[];
  reason?: string;
}
