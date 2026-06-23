import { useEffect, useMemo, useState } from 'react';

import { EmptyState, LoadingState } from '@sdkwork/notary-h5-commons';
import { AppLayout } from '@sdkwork/notary-h5-shell';

import { createNotaryH5Service, type NotaryH5DashboardStats, type NotaryH5TaskSummary } from '../services/notaryH5Service';

const STATUS_LABELS: Record<string, string> = {
  PENDING_REVIEW: 'Pending review',
  PROCESSING: 'Processing',
  COMPLETED: 'Completed',
  REJECTED: 'Rejected',
  CANCELLED: 'Cancelled',
};

function StatCard({ label, value }: { label: string; value: number | string }) {
  return (
    <div
      style={{
        flex: 1,
        minWidth: 140,
        padding: '14px 12px',
        borderRadius: 12,
        background: 'linear-gradient(180deg, #2b2b2d 0%, #252527 100%)',
        border: '1px solid rgba(255,255,255,0.06)',
      }}
    >
      <div style={{ fontSize: 12, color: '#9ca3af', marginBottom: 8 }}>{label}</div>
      <div style={{ fontSize: 24, fontWeight: 700, color: '#f9fafb' }}>{value}</div>
    </div>
  );
}

function TaskCard({ task }: { task: NotaryH5TaskSummary }) {
  return (
    <article
      style={{
        padding: '14px 16px',
        borderRadius: 14,
        background: '#1e1e20',
        border: '1px solid rgba(255,255,255,0.06)',
      }}
    >
      <div style={{ display: 'flex', justifyContent: 'space-between', gap: 12, marginBottom: 8 }}>
        <div style={{ fontSize: 15, fontWeight: 600, color: '#f3f4f6' }}>{task.title || task.id}</div>
        <span
          style={{
            fontSize: 11,
            padding: '2px 8px',
            borderRadius: 999,
            background: 'rgba(99,102,241,0.15)',
            color: '#a5b4fc',
            whiteSpace: 'nowrap',
          }}
        >
          {STATUS_LABELS[task.status] ?? task.status}
        </span>
      </div>
      <div style={{ fontSize: 12, color: '#9ca3af', display: 'grid', gap: 4 }}>
        <div>Business: {task.type || '—'}</div>
        <div>Notary: {task.notary || 'Unassigned'}</div>
        <div>Case ID: {task.id}</div>
      </div>
    </article>
  );
}

export function NotaryHomePage() {
  const service = useMemo(() => createNotaryH5Service(), []);
  const [stats, setStats] = useState<NotaryH5DashboardStats | null>(null);
  const [tasks, setTasks] = useState<NotaryH5TaskSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [accessDenied, setAccessDenied] = useState<string | null>(null);

  useEffect(() => {
    let disposed = false;

    async function load() {
      setLoading(true);
      setError(null);
      setAccessDenied(null);
      try {
        const access = await service.retrieveAccess();
        if (!access.visible) {
          if (!disposed) {
            setAccessDenied(access.reason ?? 'Notary business is not enabled for this organization.');
          }
          return;
        }

        const [nextStats, nextTasks] = await Promise.all([
          service.getDashboardStatistics(),
          service.listTasks(),
        ]);
        if (!disposed) {
          setStats(nextStats);
          setTasks(nextTasks);
        }
      } catch (loadError) {
        console.error('Failed to load notary H5 dashboard', loadError);
        if (!disposed) {
          setError('Failed to load notary workbench data.');
        }
      } finally {
        if (!disposed) {
          setLoading(false);
        }
      }
    }

    void load();
    return () => {
      disposed = true;
    };
  }, [service]);

  return (
    <AppLayout
      title="Notary Workbench"
      subtitle="Mobile H5 surface backed by the composed notary app SDK"
      footer={(
        <nav
          style={{
            position: 'fixed',
            left: 0,
            right: 0,
            bottom: 0,
            display: 'grid',
            gridTemplateColumns: '1fr',
            borderTop: '1px solid rgba(255,255,255,0.08)',
            background: '#1a1a1c',
          }}
        >
          <button
            type="button"
            style={{
              padding: '14px 12px',
              background: 'transparent',
              border: 0,
              color: '#818cf8',
              fontWeight: 600,
              fontSize: 13,
            }}
          >
            Workbench
          </button>
        </nav>
      )}
    >
      {loading ? <LoadingState label="Syncing notary data…" /> : null}
      {!loading && accessDenied ? (
        <EmptyState title="Notary access unavailable" description={accessDenied} />
      ) : null}
      {!loading && error ? (
        <EmptyState title="Unable to load workbench" description={error} />
      ) : null}
      {!loading && !error && !accessDenied && stats ? (
        <>
          <section
            style={{
              display: 'grid',
              gridTemplateColumns: 'repeat(2, minmax(0, 1fr))',
              gap: 12,
              marginBottom: 20,
            }}
          >
            <StatCard label="Pending review" value={stats.pendingCount} />
            <StatCard label="Completed today" value={stats.completedCount} />
            <StatCard label="Anomalies" value={stats.rejectedCount} />
            <StatCard label="Monthly volume" value={stats.totalCount} />
          </section>

          <section>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 12 }}>
              <h2 style={{ margin: 0, fontSize: 16, color: '#f3f4f6' }}>Recent cases</h2>
              <span style={{ fontSize: 12, color: '#9ca3af' }}>{tasks.length} loaded</span>
            </div>
            {tasks.length === 0 ? (
              <EmptyState title="No cases yet" description="Create a case from the desktop workbench or API to see it here." />
            ) : (
              <div style={{ display: 'grid', gap: 12 }}>
                {tasks.map((task) => (
                  <TaskCard key={task.id} task={task} />
                ))}
              </div>
            )}
          </section>
        </>
      ) : null}
    </AppLayout>
  );
}
