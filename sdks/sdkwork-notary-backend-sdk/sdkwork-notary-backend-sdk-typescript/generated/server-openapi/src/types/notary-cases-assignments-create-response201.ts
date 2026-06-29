import type { NotaryCaseAssignment } from './notary-case-assignment';

export interface NotaryCasesAssignmentsCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
