import type { NotaryMatter } from '@sdkwork/notary-pc-admin-core';

import type { NotaryMatterMessages } from '../i18n';

export interface MatterStatusBadgeProps {
  messages: NotaryMatterMessages;
  status: NotaryMatter['status'];
}

export function MatterStatusBadge({ messages, status }: MatterStatusBadgeProps) {
  const label = {
    active: messages.statusActive,
    draft: messages.statusDraft,
    inactive: messages.statusInactive,
  }[status];

  return <span className={`notary-matter-status is-${status}`}>{label}</span>;
}
