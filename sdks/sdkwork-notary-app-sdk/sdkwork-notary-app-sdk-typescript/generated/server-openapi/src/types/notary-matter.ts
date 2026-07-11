export interface NotaryMatter {
  skuId: string;
  spuId: string;
  skuNo: string;
  title: string;
  description?: string | null;
  /** Major-unit decimal amount projected from the Merchandise owner smallest-unit representation. */
  priceAmount: string;
  currencyCode: string;
  status: 'active' | 'inactive' | 'draft';
  spec?: Record<string, unknown>;
}
