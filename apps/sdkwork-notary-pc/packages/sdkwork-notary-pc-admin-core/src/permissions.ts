import type { NotaryPcAdminOperator } from './operator';

export function hasNotaryPcAdminPermission(
  operator: NotaryPcAdminOperator,
  permission: string,
): boolean {
  return operator.permissions.includes('*') || operator.permissions.includes(permission);
}

export function hasAnyNotaryPcAdminPermission(
  operator: NotaryPcAdminOperator,
  permissions: readonly string[],
): boolean {
  return permissions.some((permission) => hasNotaryPcAdminPermission(operator, permission));
}

export function hasAllNotaryPcAdminPermissions(
  operator: NotaryPcAdminOperator,
  permissions: readonly string[],
): boolean {
  return permissions.every((permission) => hasNotaryPcAdminPermission(operator, permission));
}
