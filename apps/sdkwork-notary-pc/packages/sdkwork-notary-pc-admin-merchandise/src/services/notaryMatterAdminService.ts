import type {
  CreateNotaryMatterRequest,
  NotaryMatter,
  NotaryMatterPage,
  SdkworkNotaryBackendClient,
  UpdateNotaryMatterRequest,
} from '@sdkwork/notary-pc-admin-core';
import { uuid } from '@sdkwork/utils';

type BackendMattersApi = SdkworkNotaryBackendClient['notary']['matters'];

export type NotaryMatterListQuery = NonNullable<
  Parameters<BackendMattersApi['management']['list']>[0]
>;

export interface NotaryMatterBackendPort {
  notary: {
    matters: {
      create: BackendMattersApi['create'];
      management: Pick<BackendMattersApi['management'], 'list'>;
      update: BackendMattersApi['update'];
    };
  };
}

export interface NotaryMatterAdminService {
  list(input?: NotaryMatterListQuery): Promise<NotaryMatterPage>;
  create(input: CreateNotaryMatterRequest, idempotencyKey?: string): Promise<NotaryMatter>;
  update(skuId: string, input: UpdateNotaryMatterRequest): Promise<NotaryMatter>;
  updateStatus(skuId: string, status: NotaryMatter['status']): Promise<NotaryMatter>;
}

export function createNotaryMatterAdminService(
  backendClient: NotaryMatterBackendPort,
): NotaryMatterAdminService {
  return {
    list(input) {
      return backendClient.notary.matters.management.list(input);
    },
    create(input, idempotencyKey = uuid()) {
      return backendClient.notary.matters.create(input, { idempotencyKey });
    },
    update(skuId, input) {
      return backendClient.notary.matters.update(skuId, input);
    },
    updateStatus(skuId, status) {
      return backendClient.notary.matters.update(skuId, { status });
    },
  };
}
