import type { ReactNode } from 'react';

import { hasAllNotaryPcAdminPermissions } from './permissions';
import { useNotaryPcAdminOperator } from './AdminRuntimeProvider';

export interface NotaryPcAdminPermissionGateProps {
  children: ReactNode;
  permissions: readonly string[];
  fallback?: ReactNode;
}

export function NotaryPcAdminPermissionGate({
  children,
  fallback = null,
  permissions,
}: NotaryPcAdminPermissionGateProps) {
  const operator = useNotaryPcAdminOperator();
  return hasAllNotaryPcAdminPermissions(operator, permissions) ? children : fallback;
}
