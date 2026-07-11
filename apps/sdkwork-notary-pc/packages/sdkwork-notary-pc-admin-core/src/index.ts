export {
  createNotaryPcAdminBackendSdkClient,
  getNotaryPcAdminBackendSdkClient,
  initNotaryPcAdminBackendSdkClient,
  resetNotaryPcAdminBackendSdkClient,
} from './sdk/backendSdk';
export type { NotaryPcAdminBackendSdkConfig } from './sdk/backendSdk';

export {
  NotaryPcAdminRuntimeProvider,
  useNotaryPcAdminBackendSdkClient,
  useNotaryPcAdminOperator,
} from './AdminRuntimeProvider';
export type { NotaryPcAdminRuntimeProviderProps } from './AdminRuntimeProvider';

export { NotaryPcAdminPermissionGate } from './AdminPermissionGate';
export type { NotaryPcAdminPermissionGateProps } from './AdminPermissionGate';

export { normalizeNotaryPcAdminOperator } from './operator';
export type { NotaryPcAdminOperator } from './operator';

export {
  hasAllNotaryPcAdminPermissions,
  hasAnyNotaryPcAdminPermission,
  hasNotaryPcAdminPermission,
} from './permissions';

export { createNotaryPcAdminAuditContext } from './audit';
export type { NotaryPcAdminAuditContext } from './audit';

export type {
  CreateNotaryMatterRequest,
  NotaryMatter,
  NotaryMatterPage,
  SdkworkNotaryBackendClient,
  UpdateNotaryMatterRequest,
} from '@sdkwork/notary-backend-sdk';
