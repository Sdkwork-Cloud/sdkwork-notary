import { LoaderCircle, Pencil, Power } from 'lucide-react';
import type { NotaryMatter } from '@sdkwork/notary-pc-admin-core';
import { formatMoney } from '@sdkwork/utils/money';

import type { NotaryMatterMessages } from '../i18n';
import { MatterStatusBadge } from './MatterStatusBadge';

export interface MatterTableProps {
  canUpdate: boolean;
  items: readonly NotaryMatter[];
  language: string;
  messages: NotaryMatterMessages;
  mutatingSkuIds: ReadonlySet<string>;
  onEdit(item: NotaryMatter): void;
  onToggleStatus(item: NotaryMatter): void;
}

function formatMatterPrice(item: NotaryMatter, language: string): string {
  return (
    formatMoney(item.priceAmount, {
      currency: item.currencyCode,
      locale: language,
      mode: 'symbol',
    }) ?? `${item.currencyCode} ${item.priceAmount}`
  );
}

export function MatterTable({
  canUpdate,
  items,
  language,
  messages,
  mutatingSkuIds,
  onEdit,
  onToggleStatus,
}: MatterTableProps) {
  return (
    <div className="notary-matter-table-frame">
      <table className="notary-matter-table">
        <thead>
          <tr>
            <th>{messages.columnMatter}</th>
            <th>{messages.columnSku}</th>
            <th>{messages.columnPrice}</th>
            <th>{messages.columnStatus}</th>
            <th className="notary-matter-actions-column">{messages.columnActions}</th>
          </tr>
        </thead>
        <tbody>
          {items.map((item) => {
            const mutating = mutatingSkuIds.has(item.skuId);
            const statusAction = item.status === 'active'
              ? messages.deactivateAction
              : messages.activateAction;
            return (
              <tr key={item.skuId}>
                <td>
                  <strong className="notary-matter-title">{item.title}</strong>
                  <span className="notary-matter-description">
                    {item.description || messages.descriptionEmpty}
                  </span>
                </td>
                <td>
                  <code className="notary-matter-sku">{item.skuNo}</code>
                </td>
                <td>
                  <span className="notary-matter-price">{formatMatterPrice(item, language)}</span>
                  {item.originalPriceAmount ? (
                    <span className="notary-matter-original-price">
                      {item.currencyCode} {item.originalPriceAmount}
                    </span>
                  ) : null}
                </td>
                <td>
                  <MatterStatusBadge messages={messages} status={item.status} />
                </td>
                <td className="notary-matter-row-actions">
                  {canUpdate ? (
                    <>
                      <button
                        className="notary-matter-icon-button"
                        type="button"
                        disabled={mutating}
                        title={messages.editAction}
                        aria-label={`${messages.editAction}: ${item.title}`}
                        onClick={() => onEdit(item)}
                      >
                        <Pencil aria-hidden="true" size={15} />
                      </button>
                      <button
                        className="notary-matter-icon-button"
                        type="button"
                        disabled={mutating}
                        title={statusAction}
                        aria-label={`${statusAction}: ${item.title}`}
                        onClick={() => onToggleStatus(item)}
                      >
                        {mutating ? (
                          <LoaderCircle aria-hidden="true" className="is-spinning" size={15} />
                        ) : (
                          <Power aria-hidden="true" size={15} />
                        )}
                      </button>
                    </>
                  ) : (
                    <span aria-label={messages.updatePermissionDenied}>-</span>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
