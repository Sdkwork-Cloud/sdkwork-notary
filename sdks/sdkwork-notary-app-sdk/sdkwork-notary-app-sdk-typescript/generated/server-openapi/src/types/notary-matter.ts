export interface NotaryMatter {
  skuId: string;
  spuId: string;
  skuNo: string;
  title: string;
  description?: string;
  priceAmount: string;
  currencyCode: string;
  status: 'active' | 'inactive' | 'draft';
  spec?: Record<string, unknown>;
}
