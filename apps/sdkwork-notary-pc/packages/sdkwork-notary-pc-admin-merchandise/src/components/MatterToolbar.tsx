import { Plus, RefreshCw, Search } from 'lucide-react';

import type { NotaryMatterMessages } from '../i18n';
import type { NotaryMatterStatusFilter } from '../types/matterViewModels';

export interface MatterToolbarProps {
  canCreate: boolean;
  loading: boolean;
  messages: NotaryMatterMessages;
  pageSize: number;
  searchValue: string;
  status: NotaryMatterStatusFilter;
  onCreate(): void;
  onPageSizeChange(pageSize: number): void;
  onRefresh(): void;
  onSearchChange(value: string): void;
  onStatusChange(status: NotaryMatterStatusFilter): void;
}

export function MatterToolbar({
  canCreate,
  loading,
  messages,
  onCreate,
  onPageSizeChange,
  onRefresh,
  onSearchChange,
  onStatusChange,
  pageSize,
  searchValue,
  status,
}: MatterToolbarProps) {
  return (
    <div className="notary-matter-toolbar">
      <label className="notary-matter-search">
        <Search aria-hidden="true" size={16} strokeWidth={1.8} />
        <input
          aria-label={messages.searchPlaceholder}
          type="search"
          value={searchValue}
          placeholder={messages.searchPlaceholder}
          onChange={(event) => onSearchChange(event.target.value)}
        />
      </label>

      <label className="notary-matter-filter">
        <span>{messages.statusFilter}</span>
        <select
          value={status}
          onChange={(event) => onStatusChange(event.target.value as NotaryMatterStatusFilter)}
        >
          <option value="all">{messages.allStatuses}</option>
          <option value="draft">{messages.statusDraft}</option>
          <option value="active">{messages.statusActive}</option>
          <option value="inactive">{messages.statusInactive}</option>
        </select>
      </label>

      <label className="notary-matter-filter">
        <span>{messages.pageSize}</span>
        <select
          value={pageSize}
          onChange={(event) => onPageSizeChange(Number(event.target.value))}
        >
          {[20, 50, 100].map((size) => (
            <option key={size} value={size}>
              {size}
            </option>
          ))}
        </select>
      </label>

      <div className="notary-matter-toolbar-actions">
        <button
          className="notary-matter-button is-secondary"
          type="button"
          disabled={loading}
          title={messages.refresh}
          onClick={onRefresh}
        >
          <RefreshCw aria-hidden="true" className={loading ? 'is-spinning' : ''} size={16} />
          <span>{messages.refresh}</span>
        </button>
        <button
          className="notary-matter-button is-primary"
          type="button"
          disabled={!canCreate}
          title={canCreate ? messages.create : messages.createPermissionDenied}
          onClick={onCreate}
        >
          <Plus aria-hidden="true" size={16} />
          <span>{messages.create}</span>
        </button>
      </div>
    </div>
  );
}
