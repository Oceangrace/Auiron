import type { DataKind } from './data-kind';

export interface CatalogField {
  readonly path: string;
  readonly kind: DataKind;
}
