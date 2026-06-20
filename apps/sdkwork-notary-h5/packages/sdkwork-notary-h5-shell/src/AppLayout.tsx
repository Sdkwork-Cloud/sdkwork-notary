import type { ReactNode } from 'react';

export interface AppLayoutProps {
  title: string;
  subtitle?: string;
  children: ReactNode;
  footer?: ReactNode;
}

export function AppLayout({ title, subtitle, children, footer }: AppLayoutProps) {
  return (
    <div
      style={{
        minHeight: '100vh',
        display: 'flex',
        flexDirection: 'column',
        background: '#111113',
      }}
    >
      <header
        style={{
          position: 'sticky',
          top: 0,
          zIndex: 10,
          padding: '16px 16px 12px',
          background: 'rgba(30, 30, 32, 0.96)',
          borderBottom: '1px solid rgba(255,255,255,0.06)',
          backdropFilter: 'blur(8px)',
        }}
      >
        <div style={{ fontSize: 18, fontWeight: 600, color: '#f9fafb' }}>{title}</div>
        {subtitle ? <div style={{ fontSize: 12, color: '#9ca3af', marginTop: 4 }}>{subtitle}</div> : null}
      </header>
      <main style={{ flex: 1, padding: '16px 16px 88px' }}>{children}</main>
      {footer}
    </div>
  );
}
