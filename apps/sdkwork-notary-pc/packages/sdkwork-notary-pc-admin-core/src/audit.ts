import type { NotaryPcAdminOperator } from './operator';

export interface NotaryPcAdminAuditContext {
  action: string;
  occurredAt: string;
  operatorId: string;
  resourceType: string;
  organizationId?: string;
  resourceId?: string;
  tenantId?: string;
}

export function createNotaryPcAdminAuditContext(
  operator: NotaryPcAdminOperator,
  input: Pick<NotaryPcAdminAuditContext, 'action' | 'resourceType' | 'resourceId'>,
): NotaryPcAdminAuditContext {
  return {
    action: input.action,
    occurredAt: new Date().toISOString(),
    operatorId: operator.operatorId,
    organizationId: operator.organizationId,
    resourceId: input.resourceId,
    resourceType: input.resourceType,
    tenantId: operator.tenantId,
  };
}
