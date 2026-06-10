export interface NotaryCaseSummary {
  totalCount: number;
  pendingReviewCount: number;
  processingCount: number;
  completedCount: number;
  rejectedCount: number;
  feeAmountTotal?: string;
}
