import { BriefcaseBusiness, ChevronRight, ShieldCheck } from 'lucide-react';
import { NavLink, Outlet } from 'react-router-dom';
import {
  hasNotaryPcAdminPermission,
  useNotaryPcAdminOperator,
} from '@sdkwork/notary-pc-admin-core';

import { getNotaryPcAdminShellMessages } from '../i18n';
import {
  NOTARY_PC_ADMIN_NAVIGATION,
  NOTARY_PC_ADMIN_MATTERS_PATH,
} from '../routes/adminRouteManifest';
import '../styles/adminShell.css';

export function NotaryPcAdminLayout() {
  const operator = useNotaryPcAdminOperator();
  const messages = getNotaryPcAdminShellMessages();
  const matterNavigation = NOTARY_PC_ADMIN_NAVIGATION.find(
    (item) => item.path === NOTARY_PC_ADMIN_MATTERS_PATH,
  );
  const canReadMatters = matterNavigation
    ? hasNotaryPcAdminPermission(operator, matterNavigation.permission)
    : false;

  return (
    <div className="notary-admin-shell">
      <aside className="notary-admin-sidebar" aria-label={messages.surface}>
        <div className="notary-admin-brand">
          <div className="notary-admin-brand-mark" aria-hidden="true">
            <ShieldCheck size={20} strokeWidth={1.9} />
          </div>
          <div>
            <strong>{messages.brand}</strong>
            <span>{messages.surface}</span>
          </div>
        </div>

        <nav className="notary-admin-navigation" aria-label={messages.operations}>
          <span className="notary-admin-navigation-label">{messages.operations}</span>
          {canReadMatters ? (
            <NavLink
              className={({ isActive }) =>
                `notary-admin-navigation-link${isActive ? ' is-active' : ''}`
              }
              to={NOTARY_PC_ADMIN_MATTERS_PATH}
            >
              <BriefcaseBusiness aria-hidden="true" size={17} strokeWidth={1.8} />
              <span>{messages.matters}</span>
              <ChevronRight className="notary-admin-navigation-chevron" aria-hidden="true" size={15} />
            </NavLink>
          ) : null}
        </nav>

        <div className="notary-admin-operator">
          <span>{messages.operator}</span>
          <strong title={operator.operatorId}>{operator.displayName}</strong>
          {operator.organizationId ? <small>{operator.organizationId}</small> : null}
        </div>
      </aside>

      <main className="notary-admin-main">
        <Outlet />
      </main>
    </div>
  );
}
