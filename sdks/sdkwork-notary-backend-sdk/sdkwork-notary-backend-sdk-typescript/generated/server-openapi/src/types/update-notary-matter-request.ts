import type { NotaryMatterStatus } from './notary-matter-status';

export interface UpdateNotaryMatterRequest {
  title?: string;
  description?: string | null;
  /** Major-unit decimal amount. currencyCode is required whenever this field is updated. */
  priceAmount?: string;
  /** Major-unit comparison amount. Null clears the comparison price; currencyCode is required when setting a value. */
  originalPriceAmount?: string | null;
  currencyCode?: string;
  status?: NotaryMatterStatus;
  spec?: Record<string, unknown>;
}
