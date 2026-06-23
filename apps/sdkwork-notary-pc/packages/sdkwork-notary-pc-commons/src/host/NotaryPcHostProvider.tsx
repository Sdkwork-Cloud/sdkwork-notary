import type { ReactNode } from 'react';
import { useMemo } from 'react';

import type { NotaryPcHostAdapter } from './notaryPcHost';
import { getNotaryPcHost } from './notaryPcHost';

export function useNotaryPcHost(): NotaryPcHostAdapter {
  return useMemo(() => getNotaryPcHost(), []);
}

export function NotaryPcHostProvider({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
