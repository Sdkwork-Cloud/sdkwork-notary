export interface PartyVideoInvite {
  inviteId: string;
  caseId: string;
  partyId: string;
  partyName?: string;
  purpose?: 'identity_verification' | 'material_confirmation' | 'remote_inquiry';
  conversationId: string;
  inviteUrl: string;
  expiresAt: string;
  driveSpaceId: string;
  driveSpaceType: 'notary';
  driveFolderNodeId: string;
}
