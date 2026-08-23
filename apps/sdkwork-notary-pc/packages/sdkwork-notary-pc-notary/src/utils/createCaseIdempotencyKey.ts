import { uuid } from '@sdkwork/utils/id';

export function createNotaryCaseIntentIdempotencyKey(): string {
  return uuid();
}

export function resolveNotaryCaseIdempotencyKey(value?: string): string {
  const callerKey = value?.trim();
  return callerKey || createNotaryCaseIntentIdempotencyKey();
}
