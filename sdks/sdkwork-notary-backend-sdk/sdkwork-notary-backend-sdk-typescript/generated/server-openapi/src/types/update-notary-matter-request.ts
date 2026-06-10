export interface UpdateNotaryMatterRequest {
  title?: string;
  description?: string;
  priceAmount?: string;
  originalPriceAmount?: string;
  currencyCode?: string;
  status?: 'draft' | 'active' | 'inactive';
  spec?: Record<string, unknown>;
}
