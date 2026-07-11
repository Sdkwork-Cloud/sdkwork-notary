import type { NotaryMatterStatus } from './notary-matter-status';

export interface NotaryMatter {
  skuId: string;
  spuId: string;
  skuNo: string;
  title: string;
  description?: string | null;
  /** Major-unit decimal amount. The server converts it exactly to the Merchandise owner smallest-unit representation using currencyCode. */
  priceAmount: string;
  /** Optional major-unit comparison amount. Null means no comparison price is configured. */
  originalPriceAmount?: string | null;
  currencyCode: string;
  status: NotaryMatterStatus;
  spec?: Record<string, unknown>;
}
