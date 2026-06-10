export interface NotaryCaseAssignment {
  id: string;
  caseId: string;
  organizationMembershipId: string;
  userId: string;
  displayName?: string;
  assignmentRole: 'primary_notary' | 'assistant' | 'reviewer' | 'approver';
  status: 'active' | 'released';
  assignedAt: string;
}
