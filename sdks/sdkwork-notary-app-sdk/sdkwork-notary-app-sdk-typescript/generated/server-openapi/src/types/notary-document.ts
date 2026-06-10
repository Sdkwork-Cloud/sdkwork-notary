import type { NotaryDocumentCategory } from './notary-document-category';

export interface NotaryDocument {
  nodeId?: string;
  driveSpaceId?: string;
  driveSpaceType?: 'notary';
  parentNodeId?: string;
  name: string;
  size: string;
  status: 'verified' | 'pending' | 'error';
  category: NotaryDocumentCategory;
  materialCode?: string;
  partyId?: string;
  mediaResource?: Record<string, unknown>;
}
