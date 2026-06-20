import { getNotaryH5ComposedApi } from '@sdkwork/notary-h5-core';

export interface NotaryH5DashboardStats {
  pendingCount: number;
  completedCount: number;
  rejectedCount: number;
  totalCount: number;
}

export interface NotaryH5TaskSummary {
  id: string;
  title: string;
  type: string;
  status: string;
  notary?: string;
  createTime?: string;
}

function asRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' ? value as Record<string, unknown> : {};
}

function numberValue(value: unknown): number {
  return typeof value === 'number' && Number.isFinite(value) ? value : 0;
}

function stringValue(value: unknown): string {
  return typeof value === 'string' ? value : '';
}

function extractItems(value: unknown): Record<string, unknown>[] {
  if (Array.isArray(value)) {
    return value.map((item) => asRecord(item));
  }
  const record = asRecord(value);
  const items = record.items;
  return Array.isArray(items) ? items.map((item) => asRecord(item)) : [];
}

export interface NotaryH5Service {
  getDashboardStatistics(): Promise<NotaryH5DashboardStats>;
  listTasks(filters?: { searchTerm?: string; status?: string; pageSize?: number }): Promise<NotaryH5TaskSummary[]>;
}

export function createNotaryH5Service(): NotaryH5Service {
  const api = getNotaryH5ComposedApi();

  return {
    async getDashboardStatistics() {
      const response = asRecord(await api.getDashboardStatistics());
      const pendingReviewQueue = asRecord(response.pendingReviewQueue);
      const todayCompleted = asRecord(response.todayCompleted);
      const anomalyIntercepted = asRecord(response.anomalyIntercepted);
      const monthlyPreservationTotal = asRecord(response.monthlyPreservationTotal);

      return {
        pendingCount: numberValue(pendingReviewQueue.count),
        completedCount: numberValue(todayCompleted.count),
        rejectedCount: numberValue(anomalyIntercepted.count),
        totalCount: numberValue(monthlyPreservationTotal.count),
      };
    },

    async listTasks(filters) {
      const response = await api.listCases({
        q: filters?.searchTerm,
        status: filters?.status && filters.status !== 'ALL' ? filters.status : undefined,
        pageSize: filters?.pageSize ?? 20,
      });

      return extractItems(response).map((record) => ({
        id: stringValue(record.caseId ?? record.id),
        title: stringValue(record.title),
        type: stringValue(record.businessType ?? record.type ?? record.matterTitle),
        status: stringValue(record.status),
        notary: stringValue(record.primaryNotaryName ?? record.notary),
        createTime: stringValue(record.createTime ?? record.createdAt ?? record.submittedAt),
      }));
    },
  };
}
