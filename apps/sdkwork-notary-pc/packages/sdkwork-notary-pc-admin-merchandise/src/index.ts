export { NotaryMatterManagementPage } from './pages/NotaryMatterManagementPage';

export { createNotaryMatterAdminService } from './services/notaryMatterAdminService';
export type {
  NotaryMatterAdminService,
  NotaryMatterBackendPort,
  NotaryMatterListQuery,
} from './services/notaryMatterAdminService';

export {
  buildCreateNotaryMatterRequest,
  buildUpdateNotaryMatterRequest,
  createEmptyNotaryMatterDraft,
  createNotaryMatterDraft,
  hasNotaryMatterFormErrors,
  validateNotaryMatterDraft,
} from './services/matterForm';

export { NOTARY_MATTER_PERMISSIONS } from './permissions';
export { getNotaryMatterMessages, interpolateNotaryMatterMessage } from './i18n';
export type { NotaryMatterMessages } from './i18n';
export type {
  NotaryMatterFormDraft,
  NotaryMatterFormErrors,
  NotaryMatterListFilters,
  NotaryMatterStatus,
  NotaryMatterStatusFilter,
} from './types/matterViewModels';
