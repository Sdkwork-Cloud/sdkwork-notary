export interface NotaryStatistics {
  pendingReviewQueue: Record<string, unknown>;
  todayCompleted: Record<string, unknown>;
  anomalyIntercepted: Record<string, unknown>;
  monthlyPreservationTotal: Record<string, unknown>;
  /** When the statistics were computed */
  timestamp?: string;
}
