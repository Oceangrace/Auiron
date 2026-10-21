import type { CatalogScope } from './catalog-scope';

export interface CatalogSearch {
  readonly matchIds?: readonly number[];
  readonly scope: CatalogScope;
  readonly contextKey: string | null;
  readonly text: string;
  readonly category: string | null;
}
