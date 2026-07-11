import { lazy, Suspense } from 'react';
import { Navigate, Route, Routes } from 'react-router-dom';
import { NotaryPcAdminPermissionGate } from '@sdkwork/notary-pc-admin-core';
import { NOTARY_MATTER_PERMISSIONS } from '@sdkwork/notary-pc-admin-merchandise';

import { NotaryPcAdminAccessDenied } from '../components/AdminAccessDenied';
import { NotaryPcAdminLayout } from '../components/AdminShellLayout';
import { getNotaryPcAdminShellMessages } from '../i18n';
import {
  NOTARY_PC_ADMIN_MATTERS_PATH,
  NOTARY_PC_ADMIN_ROOT_PATH,
} from './adminRouteManifest';

const LazyMatterManagementPage = lazy(() =>
  import('@sdkwork/notary-pc-admin-merchandise').then((module) => ({
    default: module.NotaryMatterManagementPage,
  })),
);

function AdminRouteLoadingState() {
  const messages = getNotaryPcAdminShellMessages();
  return (
    <div className="notary-admin-route-loading" role="status" aria-live="polite">
      <span className="notary-admin-route-spinner" aria-hidden="true" />
      {messages.loading}
    </div>
  );
}

function MatterRoute() {
  return (
    <NotaryPcAdminPermissionGate
      permissions={[NOTARY_MATTER_PERMISSIONS.read]}
      fallback={<NotaryPcAdminAccessDenied />}
    >
      <Suspense fallback={<AdminRouteLoadingState />}>
        <LazyMatterManagementPage />
      </Suspense>
    </NotaryPcAdminPermissionGate>
  );
}

export function NotaryPcAdminRoutes() {
  return (
    <Routes>
      <Route path={NOTARY_PC_ADMIN_ROOT_PATH} element={<NotaryPcAdminLayout />}>
        <Route index element={<Navigate replace to={NOTARY_PC_ADMIN_MATTERS_PATH} />} />
        <Route path="notary/matters" element={<MatterRoute />} />
        <Route path="*" element={<Navigate replace to={NOTARY_PC_ADMIN_MATTERS_PATH} />} />
      </Route>
    </Routes>
  );
}
