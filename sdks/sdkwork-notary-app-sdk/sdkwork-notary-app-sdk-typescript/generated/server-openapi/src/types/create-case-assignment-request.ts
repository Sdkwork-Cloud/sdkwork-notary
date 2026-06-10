export interface CreateCaseAssignmentRequest {
  organizationMembershipId: string;
  assignmentRole: 'primary_notary' | 'assistant' | 'reviewer' | 'approver';
}
