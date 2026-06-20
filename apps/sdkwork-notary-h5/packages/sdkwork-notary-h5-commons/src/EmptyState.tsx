export interface EmptyStateProps {
  title: string;
  description?: string;
}

export function EmptyState({ title, description }: EmptyStateProps) {
  return (
    <div
      style={{
        padding: '32px 16px',
        textAlign: 'center',
        color: '#9ca3af',
      }}
    >
      <div style={{ fontSize: 15, fontWeight: 600, color: '#e5e7eb', marginBottom: 8 }}>{title}</div>
      {description ? <div style={{ fontSize: 13 }}>{description}</div> : null}
    </div>
  );
}
