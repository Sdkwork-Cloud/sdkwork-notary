export interface NotaryMatter {
  skuId: string;
  spuId: string;
  skuNo: string;
  title: string;
  description?: string;
  priceAmount: string;
  originalPriceAmount?: string;
  currencyCode: string;
  status: 'draft' | 'active' | 'inactive';
  spec?: Record<string, unknown>;
}
