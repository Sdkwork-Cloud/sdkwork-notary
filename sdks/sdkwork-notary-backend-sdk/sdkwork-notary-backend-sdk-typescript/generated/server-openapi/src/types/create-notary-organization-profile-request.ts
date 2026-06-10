export interface CreateNotaryOrganizationProfileRequest {
  organizationId: string;
  displayName?: string;
  settings?: Record<string, unknown>;
}
