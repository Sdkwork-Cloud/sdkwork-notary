import { uuid } from '@sdkwork/utils/id';
import type { NotaryTask } from '@sdkwork/notary-pc-commons';

export function getNotaryTaskDisplayNo(task: NotaryTask): string {
  return task.caseNo ?? task.caseId ?? task.id;
}

export function isNotaryTaskTerminalStatus(status: NotaryTask['status']): boolean {
  return status === 'COMPLETED'
    || status === 'REJECTED'
    || status === 'CANCELLED'
    || status === 'CREATE_FAILED';
}

export function generateClientId(prefix = 'notary'): string {
  return `${prefix}-${uuid()}`;
}
