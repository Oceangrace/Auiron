import { shallowRef } from 'vue';

export class LayerExpansion {
  private static readonly KEY = 'aurion.editor.expanded-layers';
  private readonly expanded = shallowRef<ReadonlySet<string>>(new Set());

  public constructor(
    private readonly storage: Storage,
    private readonly report: (error: unknown) => void,
  ) {}

  public load(): void {
    try {
      const saved: unknown = JSON.parse(this.storage.getItem(LayerExpansion.KEY) ?? '[]');

      if (Array.isArray(saved) === true) {
        this.expanded.value = new Set(
          saved.filter((key): key is string => typeof key === 'string'),
        );
      }
    } catch (error) {
      this.report(error);
    }
  }

  public isExpanded(documentId: string, widgetId: string | null = null): boolean {
    return this.expanded.value.has(JSON.stringify([documentId, widgetId]));
  }

  public toggle(documentId: string, widgetId: string | null = null): void {
    this.setExpanded(documentId, widgetId, this.isExpanded(documentId, widgetId) === false);
  }

  public setExpanded(documentId: string, widgetId: string | null, expanded: boolean): void {
    const key = JSON.stringify([documentId, widgetId]);

    if (this.expanded.value.has(key) === expanded) {
      return;
    }

    const next = new Set(this.expanded.value);

    if (expanded === true) {
      next.add(key);
    } else {
      next.delete(key);
    }

    this.save(next);
  }

  public collapseDocument(documentId: string): void {
    const prefix = '[' + JSON.stringify(documentId) + ',';

    this.save(new Set([...this.expanded.value].filter((key) => key.startsWith(prefix) === false)));
  }

  private save(expanded: ReadonlySet<string>): void {
    this.expanded.value = expanded;

    try {
      this.storage.setItem(LayerExpansion.KEY, JSON.stringify([...expanded]));
    } catch (error) {
      this.report(error);
    }
  }
}
