export interface NotaryDownloadPackage {
  packageId: string;
  caseId: string;
  driveSpaceId: string;
  driveSpaceType: 'notary';
  status: 'preparing' | 'ready' | 'failed';
  downloadUrl?: string;
  expiresAt?: string;
}
