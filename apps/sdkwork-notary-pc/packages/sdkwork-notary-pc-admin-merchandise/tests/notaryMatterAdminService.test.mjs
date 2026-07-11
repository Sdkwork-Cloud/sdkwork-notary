import assert from 'node:assert/strict';
import test from 'node:test';

import { loadTypescriptModule } from './typescriptModuleLoader.mjs';

test('matter service delegates list, create, update, and status commands to the injected backend SDK', async () => {
  const calls = [];
  const matter = {
    skuId: 'sku-1',
    spuId: 'spu-1',
    skuNo: 'NOTARY-001',
    title: 'Evidence preservation',
    priceAmount: '99.00',
    currencyCode: 'CNY',
    status: 'active',
  };
  const page = {
    items: [matter],
    pageInfo: { mode: 'cursor', pageSize: 20, hasMore: false },
  };
  const backendClient = {
    notary: {
      matters: {
        management: {
          async list(input) {
            calls.push(['list', input]);
            return page;
          },
        },
        async create(input, options) {
          calls.push(['create', input, options]);
          return matter;
        },
        async update(skuId, input) {
          calls.push(['update', skuId, input]);
          return { ...matter, ...input };
        },
      },
    },
  };
  const { createNotaryMatterAdminService } = loadTypescriptModule(
    'src/services/notaryMatterAdminService.ts',
    { '@sdkwork/utils': { uuid: () => 'generated-idempotency-key' } },
  );
  const service = createNotaryMatterAdminService(backendClient);

  await assert.doesNotReject(() => service.list({
    pageSize: 20,
    cursor: 'cursor-2',
    q: 'evidence',
    organizationId: 'org-1',
    status: 'active',
  }));
  await service.create({
    title: matter.title,
    priceAmount: matter.priceAmount,
    currencyCode: matter.currencyCode,
  }, 'intent-1');
  await service.create({
    title: matter.title,
    priceAmount: matter.priceAmount,
    currencyCode: matter.currencyCode,
  });
  await service.update('sku-1', { title: 'Updated title' });
  await service.updateStatus('sku-1', 'inactive');

  assert.deepEqual(calls[0], ['list', {
    pageSize: 20,
    cursor: 'cursor-2',
    q: 'evidence',
    organizationId: 'org-1',
    status: 'active',
  }]);
  assert.deepEqual(calls[1][2], { idempotencyKey: 'intent-1' });
  assert.deepEqual(calls[2][2], { idempotencyKey: 'generated-idempotency-key' });
  assert.deepEqual(calls[3], ['update', 'sku-1', { title: 'Updated title' }]);
  assert.deepEqual(calls[4], ['update', 'sku-1', { status: 'inactive' }]);
});
