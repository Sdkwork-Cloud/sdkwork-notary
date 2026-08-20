export interface NotaryStatistics {
  pendingReviewQueue: { count: number; estimatedProcessHours?: number; };
  todayCompleted: { count: number; comparedToYesterday?: number; };
  anomalyIntercepted: { count: number; interceptorType?: string; };
  monthlyPreservationTotal: { count: number; blockchainSyncStatus?: 'OK' | 'PENDING' | 'ERROR'; };
  /** When the statistics were computed */
  timestamp?: string;
}
