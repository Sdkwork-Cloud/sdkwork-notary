import type { FieldError } from './field-error';

export interface ProblemDetail {
  type: string;
  title: string;
  status: number;
  detail?: string;
  code?: string;
  requestId?: string;
  fields?: FieldError[];
}
