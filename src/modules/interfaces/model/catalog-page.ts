import type { CatalogItem } from './catalog-item';

export interface CatalogPage {
  readonly catalogVersion: string;
  readonly total: number;
  readonly availableCategories: readonly string[];
  readonly items: readonly CatalogItem[];
}
