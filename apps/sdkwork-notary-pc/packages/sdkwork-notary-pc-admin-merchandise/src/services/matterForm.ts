import type {
  CreateNotaryMatterRequest,
  NotaryMatter,
  UpdateNotaryMatterRequest,
} from '@sdkwork/notary-pc-admin-core';
import {
  isCurrencyCode,
  minorUnitExponent,
  normalizeWhitespace,
  trim,
} from '@sdkwork/utils';

import type {
  NotaryMatterFormDraft,
  NotaryMatterFormErrors,
} from '../types/matterViewModels';

const CURRENCY_CODE_PATTERN = /^[A-Z]{3}$/u;

function normalizeMajorUnitAmount(value: string, currencyCode: string): string | null {
  const exponent = minorUnitExponent(currencyCode);
  const amount = trim(value);
  if (exponent === null || !/^(?:0|[1-9]\d*)(?:\.\d+)?$/u.test(amount)) {
    return null;
  }

  const [integerPart, fractionPart = ''] = amount.split('.');
  if (fractionPart.length > exponent) {
    return null;
  }

  const normalizedInteger = integerPart.replace(/^0+(?=\d)/u, '');
  if (exponent === 0) {
    return fractionPart ? null : normalizedInteger;
  }

  return `${normalizedInteger}.${fractionPart.padEnd(exponent, '0')}`;
}

function majorUnitAmountAsMinorUnits(value: string, currencyCode: string): string | null {
  const normalized = normalizeMajorUnitAmount(value, currencyCode);
  const exponent = minorUnitExponent(currencyCode);
  if (normalized === null || exponent === null) {
    return null;
  }

  const digits = normalized.replace('.', '').replace(/^0+(?=\d)/u, '');
  return digits || '0';
}

function compareUnsignedIntegerStrings(left: string, right: string): number {
  if (left.length !== right.length) {
    return left.length < right.length ? -1 : 1;
  }
  return left.localeCompare(right);
}

export function createEmptyNotaryMatterDraft(): NotaryMatterFormDraft {
  return {
    currencyCode: 'CNY',
    description: '',
    originalPriceAmount: '',
    priceAmount: '',
    status: 'draft',
    title: '',
  };
}

export function createNotaryMatterDraft(matter: NotaryMatter): NotaryMatterFormDraft {
  return {
    currencyCode: matter.currencyCode,
    description: matter.description ?? '',
    originalPriceAmount: matter.originalPriceAmount
      ? normalizeMajorUnitAmount(matter.originalPriceAmount, matter.currencyCode)
        ?? matter.originalPriceAmount
      : '',
    priceAmount: normalizeMajorUnitAmount(matter.priceAmount, matter.currencyCode)
      ?? matter.priceAmount,
    status: matter.status,
    title: matter.title,
  };
}

export function validateNotaryMatterDraft(
  draft: NotaryMatterFormDraft,
): NotaryMatterFormErrors {
  const errors: NotaryMatterFormErrors = {};
  const title = normalizeWhitespace(draft.title);
  const description = trim(draft.description);
  const priceAmount = trim(draft.priceAmount);
  const originalPriceAmount = trim(draft.originalPriceAmount);
  const currencyCode = trim(draft.currencyCode).toUpperCase();

  if (!title) {
    errors.title = 'required';
  } else if (title.length > 200) {
    errors.title = 'tooLong';
  }
  if (description.length > 4000) {
    errors.description = 'tooLong';
  }
  if (!CURRENCY_CODE_PATTERN.test(currencyCode) || !isCurrencyCode(currencyCode)) {
    errors.currencyCode = 'invalid';
  }
  const normalizedPriceAmount = normalizeMajorUnitAmount(priceAmount, currencyCode);
  if (!priceAmount) {
    errors.priceAmount = 'required';
  } else if (normalizedPriceAmount === null) {
    errors.priceAmount = 'invalid';
  }
  if (originalPriceAmount) {
    const originalMinorUnits = majorUnitAmountAsMinorUnits(originalPriceAmount, currencyCode);
    const priceMinorUnits = majorUnitAmountAsMinorUnits(priceAmount, currencyCode);
    if (originalMinorUnits === null) {
      errors.originalPriceAmount = 'invalid';
    } else if (
      priceMinorUnits !== null
      && compareUnsignedIntegerStrings(originalMinorUnits, priceMinorUnits) < 0
    ) {
      errors.originalPriceAmount = 'belowPrice';
    }
  }

  return errors;
}

export function hasNotaryMatterFormErrors(errors: NotaryMatterFormErrors): boolean {
  return Object.values(errors).some(Boolean);
}

function normalizeMatterDraft(draft: NotaryMatterFormDraft): NotaryMatterFormDraft {
  return {
    ...draft,
    currencyCode: trim(draft.currencyCode).toUpperCase(),
    description: trim(draft.description),
    originalPriceAmount: trim(draft.originalPriceAmount),
    priceAmount: trim(draft.priceAmount),
    title: normalizeWhitespace(draft.title),
  };
}

export function buildCreateNotaryMatterRequest(
  draft: NotaryMatterFormDraft,
  organizationId?: string,
): CreateNotaryMatterRequest {
  const normalized = normalizeMatterDraft(draft);
  const priceAmount = normalizeMajorUnitAmount(
    normalized.priceAmount,
    normalized.currencyCode,
  ) ?? normalized.priceAmount;
  const originalPriceAmount = normalized.originalPriceAmount
    ? normalizeMajorUnitAmount(normalized.originalPriceAmount, normalized.currencyCode)
      ?? normalized.originalPriceAmount
    : undefined;
  return {
    currencyCode: normalized.currencyCode,
    description: normalized.description || undefined,
    organizationId,
    originalPriceAmount,
    priceAmount,
    status: normalized.status,
    title: normalized.title,
  };
}

export function buildUpdateNotaryMatterRequest(
  draft: NotaryMatterFormDraft,
): UpdateNotaryMatterRequest {
  const normalized = normalizeMatterDraft(draft);
  const priceAmount = normalizeMajorUnitAmount(
    normalized.priceAmount,
    normalized.currencyCode,
  ) ?? normalized.priceAmount;
  const originalPriceAmount = normalized.originalPriceAmount
    ? normalizeMajorUnitAmount(normalized.originalPriceAmount, normalized.currencyCode)
      ?? normalized.originalPriceAmount
    : null;
  return {
    currencyCode: normalized.currencyCode,
    description: normalized.description || null,
    originalPriceAmount,
    priceAmount,
    status: normalized.status,
    title: normalized.title,
  };
}
