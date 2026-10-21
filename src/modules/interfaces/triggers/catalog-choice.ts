import type { CatalogItem } from '../model/catalog-item';
import type { CatalogForm } from '../model/catalog-form';

export class CatalogChoice {
  public all(item: CatalogItem): CatalogForm | null {
    const form = item.forms.find((form) => form.key.endsWith(':any-supported')) ?? item.forms[0];

    if (form === undefined) {
      return null;
    }

    return {
      ...form,
      matchIds: [...new Set(item.forms.flatMap((form) => form.matchIds))].sort((a, b) => a - b),
    };
  }
}
