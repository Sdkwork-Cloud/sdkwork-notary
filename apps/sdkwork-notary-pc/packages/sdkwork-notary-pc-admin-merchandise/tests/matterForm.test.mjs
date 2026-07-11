import assert from 'node:assert/strict';
import test from 'node:test';

import { loadTypescriptModule } from './typescriptModuleLoader.mjs';

const strings = {
  isCurrencyCode(value) {
    return ['CNY', 'EUR', 'GBP', 'HKD', 'JPY', 'USD'].includes(value);
  },
  minorUnitExponent(value) {
    return value === 'JPY' ? 0 : strings.isCurrencyCode(value) ? 2 : null;
  },
  normalizeWhitespace(value) {
    return value.trim().split(/\s+/u).filter(Boolean).join(' ');
  },
  trim(value) {
    return value.trim();
  },
};

const form = loadTypescriptModule('src/services/matterForm.ts', {
  '@sdkwork/utils': strings,
});

test('matter form validates generated-contract limits and decimal-string input', () => {
  assert.deepEqual(form.validateNotaryMatterDraft({
    title: '',
    description: '',
    originalPriceAmount: '',
    priceAmount: '-1',
    currencyCode: 'CN',
    status: 'draft',
  }), {
    title: 'required',
    priceAmount: 'invalid',
    currencyCode: 'invalid',
  });

  assert.equal(form.hasNotaryMatterFormErrors(form.validateNotaryMatterDraft({
    title: 'Evidence preservation',
    description: 'Remote evidence preservation service',
    originalPriceAmount: '129.00',
    priceAmount: '99.99',
    currencyCode: 'CNY',
    status: 'active',
  })), false);

  assert.deepEqual(form.validateNotaryMatterDraft({
    title: 'Evidence preservation',
    description: '',
    originalPriceAmount: '99.99',
    priceAmount: '100.00',
    currencyCode: 'CNY',
    status: 'active',
  }), {
    originalPriceAmount: 'belowPrice',
  });

  assert.deepEqual(form.validateNotaryMatterDraft({
    title: 'Evidence preservation',
    description: '',
    originalPriceAmount: '',
    priceAmount: '100.00',
    currencyCode: 'JPY',
    status: 'active',
  }), {
    priceAmount: 'invalid',
  });
});

test('matter form builds generated create and update requests without arbitrary spec fields', () => {
  const draft = {
    title: '  Evidence   preservation  ',
    description: '  Service scope  ',
    originalPriceAmount: ' 129.9 ',
    priceAmount: ' 99.00 ',
    currencyCode: 'cny',
    status: 'active',
  };

  assert.deepEqual(form.buildCreateNotaryMatterRequest(draft, 'org-1'), {
    title: 'Evidence preservation',
    description: 'Service scope',
    originalPriceAmount: '129.90',
    priceAmount: '99.00',
    currencyCode: 'CNY',
    organizationId: 'org-1',
    status: 'active',
  });
  assert.deepEqual(form.buildUpdateNotaryMatterRequest(draft), {
    title: 'Evidence preservation',
    description: 'Service scope',
    originalPriceAmount: '129.90',
    priceAmount: '99.00',
    currencyCode: 'CNY',
    status: 'active',
  });
  assert.equal('spec' in form.buildCreateNotaryMatterRequest(draft), false);

  const withoutOptionalValues = {
    ...draft,
    description: ' ',
    originalPriceAmount: ' ',
  };
  assert.equal(
    form.buildCreateNotaryMatterRequest(withoutOptionalValues).originalPriceAmount,
    undefined,
  );
  assert.deepEqual(
    {
      description: form.buildUpdateNotaryMatterRequest(withoutOptionalValues).description,
      originalPriceAmount:
        form.buildUpdateNotaryMatterRequest(withoutOptionalValues).originalPriceAmount,
    },
    { description: null, originalPriceAmount: null },
  );
});
