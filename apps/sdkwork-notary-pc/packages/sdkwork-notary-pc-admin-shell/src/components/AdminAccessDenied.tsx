import { LockKeyhole } from 'lucide-react';

import { getNotaryPcAdminShellMessages } from '../i18n';

export function NotaryPcAdminAccessDenied() {
  const messages = getNotaryPcAdminShellMessages();
  return (
    <section className="notary-admin-access-state" role="alert">
      <LockKeyhole aria-hidden="true" size={28} strokeWidth={1.8} />
      <h1>{messages.accessDeniedTitle}</h1>
      <p>{messages.accessDeniedDescription}</p>
    </section>
  );
}
