import { createNotaryAccessService } from './notaryAccessService';
import { getNotaryPcAppSdkClient } from './notaryAppSdk';

export type { NotaryAccessService, NotaryAccessState } from './notaryAccessService';

export const notaryPcAccessService = createNotaryAccessService(getNotaryPcAppSdkClient);
