import type { CatalogPage } from '../model/catalog-page';
import type { CatalogItem } from '../model/catalog-item';

export interface CatalogPickerState {
  readonly page: CatalogPage;
  readonly selected: CatalogItem | null;
  readonly loading: boolean;
  readonly restoring: boolean;
  readonly error: string | null;
}
