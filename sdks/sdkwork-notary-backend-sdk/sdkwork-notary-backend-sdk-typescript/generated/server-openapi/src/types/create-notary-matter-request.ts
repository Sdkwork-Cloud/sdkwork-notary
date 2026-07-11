import type { NotaryMatterStatus } from './notary-matter-status';

export interface CreateNotaryMatterRequest {
  organizationId?: string;
  title: string;
  description?: string;
  /** Major-unit decimal amount. The server converts it exactly to the Merchandise owner smallest-unit representation using currencyCode. */
  priceAmount: string;
  /** Optional major-unit comparison amount. */
  originalPriceAmount?: string;
  currencyCode: string;
  status?: NotaryMatterStatus;
  spec?: Record<string, unknown>;
}
