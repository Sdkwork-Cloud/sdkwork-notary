import { useEffect, useId, useRef, useState, type FormEvent } from 'react';
import { LoaderCircle, X } from 'lucide-react';
import type { NotaryMatter } from '@sdkwork/notary-pc-admin-core';

import type { NotaryMatterMessages } from '../i18n';
import {
  createEmptyNotaryMatterDraft,
  createNotaryMatterDraft,
  hasNotaryMatterFormErrors,
  validateNotaryMatterDraft,
} from '../services/matterForm';
import type {
  NotaryMatterFormDraft,
  NotaryMatterFormErrors,
} from '../types/matterViewModels';

export interface MatterEditorDialogProps {
  matter?: NotaryMatter;
  messages: NotaryMatterMessages;
  saving: boolean;
  onClose(): void;
  onSubmit(draft: NotaryMatterFormDraft): void;
}

function errorMessage(
  field: keyof NotaryMatterFormErrors,
  error: string | undefined,
  messages: NotaryMatterMessages,
): string | undefined {
  if (!error) {
    return undefined;
  }
  if (error === 'required') {
    return messages.requiredError;
  }
  if (field === 'title') {
    return messages.titleTooLongError;
  }
  if (field === 'description') {
    return messages.descriptionTooLongError;
  }
  if (field === 'priceAmount') {
    return messages.amountInvalidError;
  }
  if (field === 'originalPriceAmount') {
    return error === 'belowPrice'
      ? messages.originalAmountBelowPriceError
      : messages.amountInvalidError;
  }
  return messages.currencyInvalidError;
}

export function MatterEditorDialog({
  matter,
  messages,
  onClose,
  onSubmit,
  saving,
}: MatterEditorDialogProps) {
  const titleInputRef = useRef<HTMLInputElement>(null);
  const titleId = useId();
  const descriptionId = useId();
  const priceId = useId();
  const originalPriceId = useId();
  const currencyId = useId();
  const statusId = useId();
  const [draft, setDraft] = useState<NotaryMatterFormDraft>(() =>
    matter ? createNotaryMatterDraft(matter) : createEmptyNotaryMatterDraft(),
  );
  const [errors, setErrors] = useState<NotaryMatterFormErrors>({});

  useEffect(() => {
    setDraft(matter ? createNotaryMatterDraft(matter) : createEmptyNotaryMatterDraft());
    setErrors({});
    titleInputRef.current?.focus();
  }, [matter]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && !saving) {
        onClose();
      }
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [onClose, saving]);

  const updateDraft = <Key extends keyof NotaryMatterFormDraft>(
    key: Key,
    value: NotaryMatterFormDraft[Key],
  ) => {
    setDraft((current) => ({ ...current, [key]: value }));
    setErrors((current) => ({ ...current, [key]: undefined }));
  };

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const nextErrors = validateNotaryMatterDraft(draft);
    setErrors(nextErrors);
    if (!hasNotaryMatterFormErrors(nextErrors)) {
      onSubmit(draft);
    }
  };

  const dialogTitle = matter ? messages.dialogEditTitle : messages.dialogCreateTitle;

  return (
    <div
      className="notary-matter-dialog-backdrop"
      role="presentation"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget && !saving) {
          onClose();
        }
      }}
    >
      <section
        className="notary-matter-dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby={`${titleId}-dialog`}
      >
        <header className="notary-matter-dialog-header">
          <h2 id={`${titleId}-dialog`}>{dialogTitle}</h2>
          <button
            className="notary-matter-icon-button"
            type="button"
            disabled={saving}
            title={messages.close}
            aria-label={messages.close}
            onClick={onClose}
          >
            <X aria-hidden="true" size={17} />
          </button>
        </header>

        <form onSubmit={handleSubmit} noValidate>
          <div className="notary-matter-dialog-body">
            <label className="notary-matter-field" htmlFor={titleId}>
              <span>{messages.fieldTitle}</span>
              <input
                ref={titleInputRef}
                id={titleId}
                maxLength={200}
                value={draft.title}
                aria-invalid={Boolean(errors.title)}
                aria-describedby={errors.title ? `${titleId}-error` : undefined}
                placeholder={messages.titlePlaceholder}
                onChange={(event) => updateDraft('title', event.target.value)}
              />
              {errors.title ? (
                <small id={`${titleId}-error`} className="notary-matter-field-error">
                  {errorMessage('title', errors.title, messages)}
                </small>
              ) : null}
            </label>

            <label className="notary-matter-field" htmlFor={descriptionId}>
              <span>{messages.fieldDescription}</span>
              <textarea
                id={descriptionId}
                rows={5}
                maxLength={4000}
                value={draft.description}
                aria-invalid={Boolean(errors.description)}
                aria-describedby={errors.description ? `${descriptionId}-error` : undefined}
                placeholder={messages.descriptionPlaceholder}
                onChange={(event) => updateDraft('description', event.target.value)}
              />
              {errors.description ? (
                <small id={`${descriptionId}-error`} className="notary-matter-field-error">
                  {errorMessage('description', errors.description, messages)}
                </small>
              ) : null}
            </label>

            <div className="notary-matter-form-grid">
              <label className="notary-matter-field" htmlFor={priceId}>
                <span>{messages.fieldPrice}</span>
                <input
                  id={priceId}
                  inputMode="decimal"
                  value={draft.priceAmount}
                  aria-invalid={Boolean(errors.priceAmount)}
                  aria-describedby={errors.priceAmount ? `${priceId}-error` : undefined}
                  placeholder="0.00"
                  onChange={(event) => updateDraft('priceAmount', event.target.value)}
                />
                {errors.priceAmount ? (
                  <small id={`${priceId}-error`} className="notary-matter-field-error">
                    {errorMessage('priceAmount', errors.priceAmount, messages)}
                  </small>
                ) : null}
              </label>

              <label className="notary-matter-field" htmlFor={originalPriceId}>
                <span>{messages.fieldOriginalPrice}</span>
                <input
                  id={originalPriceId}
                  inputMode="decimal"
                  value={draft.originalPriceAmount}
                  aria-invalid={Boolean(errors.originalPriceAmount)}
                  aria-describedby={
                    errors.originalPriceAmount ? `${originalPriceId}-error` : undefined
                  }
                  placeholder={messages.originalPricePlaceholder}
                  onChange={(event) => updateDraft('originalPriceAmount', event.target.value)}
                />
                {errors.originalPriceAmount ? (
                  <small id={`${originalPriceId}-error`} className="notary-matter-field-error">
                    {errorMessage(
                      'originalPriceAmount',
                      errors.originalPriceAmount,
                      messages,
                    )}
                  </small>
                ) : null}
              </label>

              <label className="notary-matter-field" htmlFor={currencyId}>
                <span>{messages.fieldCurrency}</span>
                <select
                  id={currencyId}
                  value={draft.currencyCode}
                  aria-invalid={Boolean(errors.currencyCode)}
                  aria-describedby={errors.currencyCode ? `${currencyId}-error` : undefined}
                  onChange={(event) => updateDraft('currencyCode', event.target.value)}
                >
                  {['CNY', 'USD', 'EUR', 'GBP', 'HKD', 'JPY'].map((currency) => (
                    <option key={currency} value={currency}>{currency}</option>
                  ))}
                </select>
                {errors.currencyCode ? (
                  <small id={`${currencyId}-error`} className="notary-matter-field-error">
                    {errorMessage('currencyCode', errors.currencyCode, messages)}
                  </small>
                ) : null}
              </label>

              <label className="notary-matter-field" htmlFor={statusId}>
                <span>{messages.fieldStatus}</span>
                <select
                  id={statusId}
                  value={draft.status}
                  onChange={(event) =>
                    updateDraft('status', event.target.value as NotaryMatterFormDraft['status'])
                  }
                >
                  <option value="draft">{messages.statusDraft}</option>
                  <option value="active">{messages.statusActive}</option>
                  <option value="inactive">{messages.statusInactive}</option>
                </select>
              </label>
            </div>
          </div>

          <footer className="notary-matter-dialog-footer">
            <button
              className="notary-matter-button is-secondary"
              type="button"
              disabled={saving}
              onClick={onClose}
            >
              {messages.cancel}
            </button>
            <button className="notary-matter-button is-primary" type="submit" disabled={saving}>
              {saving ? <LoaderCircle aria-hidden="true" className="is-spinning" size={16} /> : null}
              <span>{saving ? messages.saving : messages.save}</span>
            </button>
          </footer>
        </form>
      </section>
    </div>
  );
}
