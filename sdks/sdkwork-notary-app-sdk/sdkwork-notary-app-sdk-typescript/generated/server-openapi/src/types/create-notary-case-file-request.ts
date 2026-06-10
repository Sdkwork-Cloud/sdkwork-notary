import type { NotaryDocumentCategory } from './notary-document-category';

export interface CreateNotaryCaseFileRequest {
  driveNodeId: string;
  category: NotaryDocumentCategory;
  reviewStatus?: 'verified' | 'pending' | 'error';
  materialCode?: string;
  partyId?: string;
  remarks?: string;
}
