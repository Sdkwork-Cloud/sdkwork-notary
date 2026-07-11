import { NOTARY_MATTER_PERMISSIONS } from '@sdkwork/notary-pc-admin-merchandise';

export const NOTARY_PC_ADMIN_ROOT_PATH = '/admin';
export const NOTARY_PC_ADMIN_MATTERS_PATH = '/admin/notary/matters';

export interface NotaryPcAdminNavigationItem {
  id: string;
  path: string;
  permission: string;
}

export const NOTARY_PC_ADMIN_NAVIGATION: readonly NotaryPcAdminNavigationItem[] = [
  {
    id: 'notary-matters',
    path: NOTARY_PC_ADMIN_MATTERS_PATH,
    permission: NOTARY_MATTER_PERMISSIONS.read,
  },
];
