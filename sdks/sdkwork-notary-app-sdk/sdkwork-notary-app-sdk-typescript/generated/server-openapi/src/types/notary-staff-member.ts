export interface NotaryStaffMember {
  membershipId: string;
  userId: string;
  employeeNo?: string;
  displayName: string;
  status: string;
  roles: string[];
  positions: string[];
  departments: string[];
  notaryStaffRole?: 'notary' | 'assistant' | 'reviewer' | 'approver';
}
