import type { ReactNode } from 'react';

export const NOTARY_PC_HOME_PATH = '/notary';

export function NotaryPcAppLayout({ children }: { children: ReactNode }) {
  return (
    <div className="notary-pc-app-layout" style={{ minHeight: '100vh', background: '#f5f6f8' }}>
      <header
        style={{
          height: 56,
          display: 'flex',
          alignItems: 'center',
          padding: '0 24px',
          borderBottom: '1px solid #e5e7eb',
          background: '#fff',
          fontWeight: 600,
        }}
      >
        SDKWork Notary PC
      </header>
      <main style={{ height: 'calc(100vh - 56px)' }}>{children}</main>
    </div>
  );
}
