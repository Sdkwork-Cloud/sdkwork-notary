export interface Party {
  id: string;
  name: string;
  role: string;
  partyType?: 'natural_person' | 'legal_person' | 'organization';
  identityType?: string;
  /** Masked identity number for display. */
  identityId: string;
  identityNoLast4?: string;
  phone?: string;
  gender?: string;
  birthDate?: string;
  address?: string;
  remarks?: string;
  identityValidDateStart?: string;
  identityValidDateEnd?: string;
  signatureUrl?: string;
  signatureNodeId?: string;
  verificationStatus?: 'pending' | 'verified' | 'failed' | 'expired';
}
