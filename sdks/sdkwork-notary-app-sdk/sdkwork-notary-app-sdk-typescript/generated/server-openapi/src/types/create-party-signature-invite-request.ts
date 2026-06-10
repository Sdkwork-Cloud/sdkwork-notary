export interface CreatePartySignatureInviteRequest {
  purpose?: 'remote_signature' | 'onsite_signature_confirmation' | 'material_signature';
  expiresInSeconds?: number;
}
