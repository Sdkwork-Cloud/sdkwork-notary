export interface CreateNotaryPartyRequest {
  name: string;
  role: string;
  partyType?: 'natural_person' | 'legal_person' | 'organization';
  identityType?: string;
  identityNo: string;
  phone?: string;
  gender?: string;
  birthDate?: string;
  address?: string;
  remarks?: string;
  identityValidDateStart?: string;
  identityValidDateEnd?: string;
}
