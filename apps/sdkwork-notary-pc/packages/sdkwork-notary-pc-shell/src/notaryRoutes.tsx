import { NotaryView } from '@sdkwork/notary-pc-notary';

import { NotaryPcAppLayout } from './AppLayout';

export function NotaryPcRoutes() {
  return (
    <NotaryPcAppLayout>
      <NotaryView />
    </NotaryPcAppLayout>
  );
}

export { NOTARY_PC_HOME_PATH } from './AppLayout';
