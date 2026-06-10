import type { NotaryMatter } from './notary-matter';
import type { PageInfo } from './page-info';

export interface NotaryMatterPage {
  items: NotaryMatter[];
  pageInfo: PageInfo;
}
