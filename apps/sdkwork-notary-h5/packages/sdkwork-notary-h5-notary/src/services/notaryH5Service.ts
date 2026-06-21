import { getNotaryH5ComposedApi } from '@sdkwork/notary-h5-core';
import { isBlank } from '@sdkwork/utils/string';

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

function coalesce(...values: Array<string | null | undefined>): string | undefined {
  for (const value of values) {
    if (!isBlank(value)) {
      return value!.trim();
    }
  }
  return undefined;
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
        id: coalesce(stringValue(record.caseId), stringValue(record.id)) ?? '',
        title: stringValue(record.title),
        type: coalesce(
          stringValue(record.businessType),
          stringValue(record.type),
          stringValue(record.matterTitle),
        ) ?? '',
        status: stringValue(record.status),
        notary: coalesce(
          stringValue(record.primaryNotaryName),
          stringValue(record.notary),
        ),
        createTime: coalesce(
          stringValue(record.createTime),
          stringValue(record.createdAt),
          stringValue(record.submittedAt),
        ),
      }));
    },
  };
}
