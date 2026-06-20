import type { ReactNode } from 'react';

export interface AuthGateProps {
  children: ReactNode;
}

/**
 * Placeholder auth gate until appbase IAM H5 login surface is wired.
 * SDK clients still receive access tokens from sessionStorage when present.
 */
export function AuthGate({ children }: AuthGateProps) {
  return <>{children}</>;
}
