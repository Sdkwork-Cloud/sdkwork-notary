import type { NotaryCase } from './notary-case';
import type { PageInfo } from './page-info';

export interface NotaryCasePage {
  items: NotaryCase[];
  pageInfo: PageInfo;
}
