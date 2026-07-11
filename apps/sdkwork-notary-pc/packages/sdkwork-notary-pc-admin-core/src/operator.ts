import { normalizeWhitespace } from '@sdkwork/utils';

export interface NotaryPcAdminOperator {
  operatorId: string;
  displayName: string;
  permissions: readonly string[];
  tenantId?: string;
  organizationId?: string;
}

export function normalizeNotaryPcAdminOperator(
  operator: NotaryPcAdminOperator,
): NotaryPcAdminOperator {
  const operatorId = normalizeWhitespace(operator.operatorId);
  if (!operatorId) {
    throw new Error('Notary PC admin operatorId is required.');
  }

  const displayName = normalizeWhitespace(operator.displayName) || operatorId;
  const permissions = Array.from(
    new Set(operator.permissions.map(normalizeWhitespace).filter(Boolean)),
  ).sort();

  return {
    ...operator,
    operatorId,
    displayName,
    permissions,
    tenantId: operator.tenantId ? normalizeWhitespace(operator.tenantId) : undefined,
    organizationId: operator.organizationId
      ? normalizeWhitespace(operator.organizationId)
      : undefined,
  };
}
