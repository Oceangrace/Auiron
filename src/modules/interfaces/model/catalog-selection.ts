import type { CatalogScope } from './catalog-scope';

export interface CatalogSelection {
  readonly catalogVersion: string;
  readonly scope: CatalogScope;
  readonly entryKey: string;
  readonly choiceKey: string;
  readonly contextKey: string | null;
}
