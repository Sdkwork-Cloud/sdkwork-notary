export interface UpdateNotaryOrganizationProfileRequest {
  status?: 'active' | 'suspended' | 'closed';
  settings?: Record<string, unknown>;
  version?: string;
}
