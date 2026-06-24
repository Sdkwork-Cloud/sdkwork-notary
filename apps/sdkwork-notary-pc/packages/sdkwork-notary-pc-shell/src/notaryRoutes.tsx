import { Suspense, lazy } from 'react';

import { LoadingState } from '@sdkwork/notary-pc-commons';

import { NotaryPcAppLayout } from './AppLayout';

const LazyNotaryView = lazy(() =>
  import('@sdkwork/notary-pc-notary').then((module) => ({ default: module.NotaryView })),
);

export function NotaryPcRoutes() {
  return (
    <NotaryPcAppLayout>
      <Suspense fallback={<LoadingState label="Loading notary workspace…" />}>
        <LazyNotaryView />
      </Suspense>
    </NotaryPcAppLayout>
  );
}

export { NOTARY_PC_HOME_PATH } from './AppLayout';
