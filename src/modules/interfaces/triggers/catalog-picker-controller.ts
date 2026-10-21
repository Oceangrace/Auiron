import { shallowReadonly, shallowRef } from 'vue';
import type { ShallowRef } from 'vue';
import type { CatalogClient } from './catalog-client';
import type { CatalogSearch } from '../model/catalog-search';
import type { CatalogScope } from '../model/catalog-scope';
import type { CatalogItem } from '../model/catalog-item';
import type { CatalogPickerState } from './catalog-picker-state';

export class CatalogPickerController {
  private readonly state = shallowRef<CatalogPickerState>({
    page: { catalogVersion: '', total: 0, availableCategories: [], items: [] },
    selected: null,
    loading: false,
    restoring: false,
    error: null,
  });

  private sequence = 0;
  private restoreSequence = 0;
  private timer: ReturnType<typeof setTimeout> | null = null;

  public constructor(private readonly client: CatalogClient) {}

  public getState(): Readonly<ShallowRef<CatalogPickerState>> {
    return shallowReadonly(this.state);
  }

  public schedule(request: CatalogSearch): void {
    const sequence = ++this.sequence;

    if (this.timer !== null) {
      clearTimeout(this.timer);
    }

    this.state.value = { ...this.state.value, loading: true };
    this.timer = setTimeout(() => {
      void this.search(request, sequence);
    }, 150);
  }

  private async search(request: CatalogSearch, sequence: number): Promise<void> {
    try {
      const page = await this.client.search(request);

      if (sequence !== this.sequence) {
        return;
      }

      this.state.value = { ...this.state.value, page, loading: false, error: null };
    } catch (error) {
      if (sequence === this.sequence) {
        this.state.value = { ...this.state.value, loading: false, error: String(error) };
      }
    }
  }

  public select(item: CatalogItem): void {
    this.restoreSequence++;
    this.state.value = { ...this.state.value, selected: item, restoring: false };
  }

  public async restore(
    scope: CatalogScope,
    contextKey: string | null,
    key: string | null,
  ): Promise<void> {
    const sequence = ++this.restoreSequence;

    this.state.value = { ...this.state.value, selected: null, restoring: key !== null };

    if (key === null) {
      return;
    }

    try {
      const record = await this.client.item(scope, contextKey, key);

      if (sequence === this.restoreSequence) {
        this.state.value = {
          ...this.state.value,
          selected: record.item,
          restoring: false,
          page: { ...this.state.value.page, catalogVersion: record.catalogVersion },
        };
      }
    } catch (error) {
      if (sequence === this.restoreSequence) {
        this.state.value = { ...this.state.value, restoring: false, error: String(error) };
      }
    }
  }

  public dispose(): void {
    this.sequence++;
    this.restoreSequence++;

    if (this.timer !== null) {
      clearTimeout(this.timer);
    }
  }
}
