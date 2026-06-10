import type { NotaryDocument } from './notary-document';
import type { PageInfo } from './page-info';

export interface NotaryDocumentList {
  items: NotaryDocument[];
  pageInfo: PageInfo;
}
