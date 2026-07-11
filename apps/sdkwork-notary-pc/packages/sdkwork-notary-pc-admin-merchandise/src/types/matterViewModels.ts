import type { NotaryMatter } from '@sdkwork/notary-pc-admin-core';

export type NotaryMatterStatus = NotaryMatter['status'];
export type NotaryMatterStatusFilter = 'all' | NotaryMatterStatus;

export interface NotaryMatterListFilters {
  organizationId?: string;
  pageSize: number;
  q: string;
  status: NotaryMatterStatusFilter;
}

export interface NotaryMatterFormDraft {
  currencyCode: string;
  description: string;
  originalPriceAmount: string;
  priceAmount: string;
  status: NotaryMatterStatus;
  title: string;
}

export interface NotaryMatterFormErrors {
  currencyCode?: string;
  description?: string;
  originalPriceAmount?: string;
  priceAmount?: string;
  title?: string;
}
