import type { NotaryDownloadPackage } from './notary-download-package';

export interface NotaryCasesDownloadPackagesCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
