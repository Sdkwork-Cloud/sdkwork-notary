import { AlertTriangle, Inbox, LoaderCircle, RotateCcw, ShieldAlert } from 'lucide-react';

export type MatterStateKind = 'denied' | 'empty' | 'error' | 'loading';

export interface MatterStatePanelProps {
  kind: MatterStateKind;
  title: string;
  description?: string;
  retryLabel?: string;
  onRetry?(): void;
}

const ICONS = {
  denied: ShieldAlert,
  empty: Inbox,
  error: AlertTriangle,
  loading: LoaderCircle,
} as const;

export function MatterStatePanel({
  description,
  kind,
  onRetry,
  retryLabel,
  title,
}: MatterStatePanelProps) {
  const Icon = ICONS[kind];
  return (
    <div className={`notary-matter-state is-${kind}`} role={kind === 'error' ? 'alert' : 'status'}>
      <Icon
        aria-hidden="true"
        className={kind === 'loading' ? 'is-spinning' : undefined}
        size={28}
        strokeWidth={1.7}
      />
      <strong>{title}</strong>
      {description ? <p>{description}</p> : null}
      {onRetry && retryLabel ? (
        <button className="notary-matter-button is-secondary" type="button" onClick={onRetry}>
          <RotateCcw aria-hidden="true" size={15} />
          <span>{retryLabel}</span>
        </button>
      ) : null}
    </div>
  );
}
