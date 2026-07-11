import { useCallback, useEffect, useRef, useState } from 'react';
import type { NotaryMatter, NotaryMatterPage } from '@sdkwork/notary-pc-admin-core';
import { DEFAULT_LIST_PAGE_SIZE } from '@sdkwork/utils';

import type {
  NotaryMatterAdminService,
  NotaryMatterListQuery,
} from '../services/notaryMatterAdminService';
import type { NotaryMatterListFilters } from '../types/matterViewModels';

const EMPTY_PAGE: NotaryMatterPage = {
  items: [],
  pageInfo: {
    mode: 'cursor',
    hasMore: false,
    pageSize: DEFAULT_LIST_PAGE_SIZE,
  },
};

export interface UseNotaryMatterListResult {
  error: unknown;
  filters: NotaryMatterListFilters;
  hasNextPage: boolean;
  hasPreviousPage: boolean;
  items: NotaryMatter[];
  loading: boolean;
  pageInfo: NotaryMatterPage['pageInfo'];
  pageNumber: number;
  goToNextPage(): void;
  goToPreviousPage(): void;
  refresh(options?: { resetPage?: boolean }): void;
  replaceItem(item: NotaryMatter): void;
  setFilters(filters: Partial<NotaryMatterListFilters>): void;
}

export function useNotaryMatterList(
  service: NotaryMatterAdminService,
  initialOrganizationId?: string,
): UseNotaryMatterListResult {
  const [filters, setFiltersState] = useState<NotaryMatterListFilters>({
    organizationId: initialOrganizationId,
    pageSize: DEFAULT_LIST_PAGE_SIZE,
    q: '',
    status: 'all',
  });
  const [page, setPage] = useState<NotaryMatterPage>(EMPTY_PAGE);
  const [pageNumber, setPageNumber] = useState(1);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<unknown>(null);
  const [revision, setRevision] = useState(0);
  const cursorByPageRef = useRef<Map<number, string | undefined>>(new Map([[1, undefined]]));
  const requestSequenceRef = useRef(0);

  const resetPagination = useCallback(() => {
    cursorByPageRef.current = new Map([[1, undefined]]);
    setPageNumber(1);
    setPage(EMPTY_PAGE);
  }, []);

  const setFilters = useCallback(
    (nextFilters: Partial<NotaryMatterListFilters>) => {
      resetPagination();
      setFiltersState((current) => ({ ...current, ...nextFilters }));
    },
    [resetPagination],
  );

  const refresh = useCallback(
    (options?: { resetPage?: boolean }) => {
      if (options?.resetPage) {
        resetPagination();
      }
      setRevision((current) => current + 1);
    },
    [resetPagination],
  );

  const replaceItem = useCallback((item: NotaryMatter) => {
    setPage((current) => ({
      ...current,
      items: current.items.map((candidate) =>
        candidate.skuId === item.skuId ? item : candidate,
      ),
    }));
  }, []);

  useEffect(() => {
    const requestSequence = requestSequenceRef.current + 1;
    requestSequenceRef.current = requestSequence;
    const cursor = cursorByPageRef.current.get(pageNumber);
    const query: NotaryMatterListQuery = {
      cursor,
      organizationId: filters.organizationId,
      pageSize: filters.pageSize,
      q: filters.q || undefined,
      status: filters.status === 'all' ? undefined : filters.status,
    };

    setLoading(true);
    setError(null);

    void service
      .list(query)
      .then((result) => {
        if (requestSequence !== requestSequenceRef.current) {
          return;
        }
        setPage(result);
        const nextCursor = result.pageInfo.nextCursor?.trim();
        if (result.pageInfo.hasMore && nextCursor) {
          cursorByPageRef.current.set(pageNumber + 1, nextCursor);
        } else {
          cursorByPageRef.current.delete(pageNumber + 1);
        }
      })
      .catch((loadError: unknown) => {
        if (requestSequence === requestSequenceRef.current) {
          setError(loadError);
          setPage((current) => ({ ...current, items: [] }));
        }
      })
      .finally(() => {
        if (requestSequence === requestSequenceRef.current) {
          setLoading(false);
        }
      });

    return () => {
      requestSequenceRef.current += 1;
    };
  }, [filters, pageNumber, revision, service]);

  const goToPreviousPage = useCallback(() => {
    setPageNumber((current) => Math.max(1, current - 1));
  }, []);

  const goToNextPage = useCallback(() => {
    setPageNumber((current) =>
      cursorByPageRef.current.has(current + 1) ? current + 1 : current,
    );
  }, []);

  return {
    error,
    filters,
    goToNextPage,
    goToPreviousPage,
    hasNextPage: Boolean(page.pageInfo.hasMore && cursorByPageRef.current.has(pageNumber + 1)),
    hasPreviousPage: pageNumber > 1,
    items: page.items,
    loading,
    pageInfo: page.pageInfo,
    pageNumber,
    refresh,
    replaceItem,
    setFilters,
  };
}
