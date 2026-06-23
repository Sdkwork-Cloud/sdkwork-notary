export interface LoadingStateProps {
  label?: string;
}

export function LoadingState({ label = 'Loading…' }: LoadingStateProps) {
  return (
    <div
      className="notary-pc-loading-state"
      style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        padding: '24px 16px',
        color: '#6b7280',
        fontSize: 14,
      }}
    >
      {label}
    </div>
  );
}
