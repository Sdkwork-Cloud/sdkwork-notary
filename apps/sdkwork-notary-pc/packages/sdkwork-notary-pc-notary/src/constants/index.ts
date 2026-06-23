/**
 * Shared constants for the notary package
 */
import { getNotaryPcHost } from '@sdkwork/notary-pc-commons';

export function getDefaultNotaryCallerAvatar(): string {
  return getNotaryPcHost().createDefaultAvatar('user');
}

export { SYSTEM_ASSIGNED_NOTARY_LABEL } from './notary';

/**
 * Empty 1x1 transparent GIF for print template placeholders
 * Used when party identity media URLs are not available
 */
export const EMPTY_NOTARY_PRINT_IMAGE_URL = 'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///ywAAAAAAQABAAACAUwAOw==';
