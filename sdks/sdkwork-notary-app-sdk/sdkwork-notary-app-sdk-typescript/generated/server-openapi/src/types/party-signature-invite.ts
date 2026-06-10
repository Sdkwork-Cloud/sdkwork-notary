export interface PartySignatureInvite {
  inviteId: string;
  caseId: string;
  partyId: string;
  partyName?: string;
  purpose?: 'remote_signature' | 'onsite_signature_confirmation' | 'material_signature';
  inviteUrl: string;
  signingUrl: string;
  expiresAt: string;
  driveSpaceId: string;
  driveSpaceType: 'notary';
  driveFolderNodeId: string;
}
