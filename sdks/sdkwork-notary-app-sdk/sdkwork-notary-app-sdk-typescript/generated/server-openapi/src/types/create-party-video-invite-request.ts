export interface CreatePartyVideoInviteRequest {
  purpose?: 'identity_verification' | 'material_confirmation' | 'remote_inquiry';
  expiresInSeconds?: number;
}
