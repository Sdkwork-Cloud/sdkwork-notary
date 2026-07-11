import { useEffect, useMemo, useState } from 'react';
import { CheckCircle2, X } from 'lucide-react';
import type { NotaryMatter } from '@sdkwork/notary-pc-admin-core';
import {
  hasNotaryPcAdminPermission,
  useNotaryPcAdminBackendSdkClient,
  useNotaryPcAdminOperator,
} from '@sdkwork/notary-pc-admin-core';
import { normalizeWhitespace, readSdkWorkProblemMessage } from '@sdkwork/utils';

import { MatterEditorDialog } from '../components/MatterEditorDialog';
import { MatterStatePanel } from '../components/MatterStatePanel';
import { MatterTable } from '../components/MatterTable';
import { MatterToolbar } from '../components/MatterToolbar';
import { useNotaryMatterList } from '../hooks/useNotaryMatterList';
import {
  getNotaryMatterMessages,
  interpolateNotaryMatterMessage,
} from '../i18n';
import { NOTARY_MATTER_PERMISSIONS } from '../permissions';
import {
  buildCreateNotaryMatterRequest,
  buildUpdateNotaryMatterRequest,
} from '../services/matterForm';
import { createNotaryMatterAdminService } from '../services/notaryMatterAdminService';
import type {
  NotaryMatterFormDraft,
  NotaryMatterStatusFilter,
} from '../types/matterViewModels';
import '../styles/matterManagement.css';

interface EditorState {
  matter?: NotaryMatter;
}

interface FeedbackState {
  kind: 'error' | 'success';
  message: string;
}

function resolveRuntimeLanguage(): string {
  if (typeof document !== 'undefined' && document.documentElement.lang) {
    return document.documentElement.lang;
  }
  return typeof navigator !== 'undefined' ? navigator.language : 'zh-CN';
}

export function NotaryMatterManagementPage() {
  const backendClient = useNotaryPcAdminBackendSdkClient();
  const operator = useNotaryPcAdminOperator();
  const service = useMemo(() => createNotaryMatterAdminService(backendClient), [backendClient]);
  const list = useNotaryMatterList(service, operator.organizationId);
  const language = resolveRuntimeLanguage();
  const messages = getNotaryMatterMessages(language);
  const [searchValue, setSearchValue] = useState('');
  const [editor, setEditor] = useState<EditorState | null>(null);
  const [saving, setSaving] = useState(false);
  const [feedback, setFeedback] = useState<FeedbackState | null>(null);
  const [mutatingSkuIds, setMutatingSkuIds] = useState<ReadonlySet<string>>(new Set());

  const canRead = hasNotaryPcAdminPermission(operator, NOTARY_MATTER_PERMISSIONS.read);
  const canCreate = hasNotaryPcAdminPermission(operator, NOTARY_MATTER_PERMISSIONS.create);
  const canUpdate = hasNotaryPcAdminPermission(operator, NOTARY_MATTER_PERMISSIONS.update);

  useEffect(() => {
    const timer = window.setTimeout(() => {
      const q = normalizeWhitespace(searchValue);
      if (q !== list.filters.q) {
        list.setFilters({ q });
      }
    }, 300);
    return () => window.clearTimeout(timer);
  }, [list.filters.q, list.setFilters, searchValue]);

  const showError = (error: unknown, fallback: string) => {
    const problem = error && typeof error === 'object' && 'problem' in error
      ? (error as { problem: unknown }).problem
      : error;
    setFeedback({ kind: 'error', message: readSdkWorkProblemMessage(problem, fallback) });
  };

  const handleSave = async (draft: NotaryMatterFormDraft) => {
    if (saving || !editor) {
      return;
    }
    setSaving(true);
    setFeedback(null);
    try {
      if (editor.matter) {
        if (!canUpdate) {
          setFeedback({ kind: 'error', message: messages.updatePermissionDenied });
          return;
        }
        const updated = await service.update(
          editor.matter.skuId,
          buildUpdateNotaryMatterRequest(draft),
        );
        list.replaceItem(updated);
        setFeedback({ kind: 'success', message: messages.updated });
      } else {
        if (!canCreate) {
          setFeedback({ kind: 'error', message: messages.createPermissionDenied });
          return;
        }
        await service.create(buildCreateNotaryMatterRequest(draft, operator.organizationId));
        list.refresh({ resetPage: true });
        setFeedback({ kind: 'success', message: messages.created });
      }
      setEditor(null);
    } catch (error) {
      showError(error, editor.matter ? messages.updateFailed : messages.createFailed);
    } finally {
      setSaving(false);
    }
  };

  const handleToggleStatus = async (matter: NotaryMatter) => {
    if (!canUpdate || mutatingSkuIds.has(matter.skuId)) {
      return;
    }
    const nextStatus: NotaryMatter['status'] = matter.status === 'active' ? 'inactive' : 'active';
    setMutatingSkuIds((current) => new Set(current).add(matter.skuId));
    setFeedback(null);
    try {
      const updated = await service.updateStatus(matter.skuId, nextStatus);
      list.replaceItem(updated);
      setFeedback({ kind: 'success', message: messages.statusUpdated });
    } catch (error) {
      showError(error, messages.statusUpdateFailed);
    } finally {
      setMutatingSkuIds((current) => {
        const next = new Set(current);
        next.delete(matter.skuId);
        return next;
      });
    }
  };

  if (!canRead) {
    return (
      <div className="notary-matter-page">
        <MatterStatePanel kind="denied" title={messages.permissionDenied} />
      </div>
    );
  }

  const hasFilters = Boolean(list.filters.q || list.filters.status !== 'all');
  const totalLabel = list.pageInfo.totalItems
    ? interpolateNotaryMatterMessage(messages.totalItems, { total: list.pageInfo.totalItems })
    : null;

  return (
    <section className="notary-matter-page">
      <header className="notary-matter-page-header">
        <div>
          <h1>{messages.title}</h1>
          {totalLabel ? <span>{totalLabel}</span> : null}
        </div>
      </header>

      {feedback ? (
        <div className={`notary-matter-feedback is-${feedback.kind}`} role="status">
          {feedback.kind === 'success' ? <CheckCircle2 aria-hidden="true" size={17} /> : null}
          <span>{feedback.message}</span>
          <button
            className="notary-matter-icon-button"
            type="button"
            title={messages.close}
            aria-label={messages.close}
            onClick={() => setFeedback(null)}
          >
            <X aria-hidden="true" size={15} />
          </button>
        </div>
      ) : null}

      <MatterToolbar
        canCreate={canCreate}
        loading={list.loading}
        messages={messages}
        pageSize={list.filters.pageSize}
        searchValue={searchValue}
        status={list.filters.status}
        onCreate={() => setEditor({})}
        onPageSizeChange={(pageSize) => list.setFilters({ pageSize })}
        onRefresh={() => list.refresh()}
        onSearchChange={setSearchValue}
        onStatusChange={(status: NotaryMatterStatusFilter) => list.setFilters({ status })}
      />

      {list.loading && list.items.length === 0 ? (
        <MatterStatePanel kind="loading" title={messages.loading} />
      ) : list.error ? (
        <MatterStatePanel
          kind="error"
          title={messages.loadFailed}
          description={readSdkWorkProblemMessage(list.error, messages.loadFailed)}
          retryLabel={messages.retry}
          onRetry={() => list.refresh()}
        />
      ) : list.items.length === 0 ? (
        <MatterStatePanel
          kind="empty"
          title={messages.emptyTitle}
          description={hasFilters ? messages.emptyFiltered : messages.emptyInitial}
        />
      ) : (
        <>
          <MatterTable
            canUpdate={canUpdate}
            items={list.items}
            language={language}
            messages={messages}
            mutatingSkuIds={mutatingSkuIds}
            onEdit={(matter) => setEditor({ matter })}
            onToggleStatus={(matter) => void handleToggleStatus(matter)}
          />
          <footer className="notary-matter-pagination">
            <button
              className="notary-matter-button is-secondary"
              type="button"
              disabled={!list.hasPreviousPage || list.loading}
              onClick={list.goToPreviousPage}
            >
              {messages.previousPage}
            </button>
            <span>
              {interpolateNotaryMatterMessage(messages.pageNumber, { page: list.pageNumber })}
            </span>
            <button
              className="notary-matter-button is-secondary"
              type="button"
              disabled={!list.hasNextPage || list.loading}
              onClick={list.goToNextPage}
            >
              {messages.nextPage}
            </button>
          </footer>
        </>
      )}

      {editor ? (
        <MatterEditorDialog
          matter={editor.matter}
          messages={messages}
          saving={saving}
          onClose={() => setEditor(null)}
          onSubmit={(draft) => void handleSave(draft)}
        />
      ) : null}
    </section>
  );
}
