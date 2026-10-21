import type { DesktopCommands } from '../../../core/desktop/desktop-commands';
import type { CatalogLabels } from '../../../core/catalog/catalog-labels';
import type { CatalogPage } from '../model/catalog-page';
import type { CatalogSearch } from '../model/catalog-search';
import type { CatalogRecord } from '../model/catalog-record';
import type { CatalogScope } from '../model/catalog-scope';
import type { FieldBossDefinition } from '../../../core/catalog/field-boss-definition';

export class CatalogClient {
  public constructor(
    private readonly desktop: DesktopCommands,
    private readonly locale: () => string,
    private readonly labels: () => CatalogLabels,
  ) {}

  public icons(): Readonly<Record<string, string>> {
    return this.labels().icons;
  }

  public fieldBosses(): readonly FieldBossDefinition[] {
    return this.labels().fieldBosses ?? [];
  }

  public npcName(id: number): string {
    return this.labels().npcs[id] ?? String(id);
  }

  public zoneName(id: number): string {
    return this.labels().zones?.[id] ?? String(id);
  }

  public async search(request: CatalogSearch): Promise<CatalogPage> {
    return this.desktop.invoke<CatalogPage>('catalog_search', {
      request: { ...request, locale: this.locale() },
    });
  }

  public async item(
    scope: CatalogScope,
    contextKey: string | null,
    key: string,
  ): Promise<CatalogRecord> {
    return this.desktop.invoke<CatalogRecord>('catalog_item', {
      scope,
      contextKey,
      key,
      locale: this.locale(),
    });
  }
}
