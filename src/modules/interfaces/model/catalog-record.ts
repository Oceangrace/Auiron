import type { CatalogItem } from './catalog-item';

export interface CatalogRecord {
  readonly catalogVersion: string;
  readonly item: CatalogItem | null;
}
