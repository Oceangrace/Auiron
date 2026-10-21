import type { CatalogLabels } from '../../../core/catalog/catalog-labels';
import type { JournalAction } from '../model/journal-action';
import { JournalActionKind } from '../model/journal-action-kind';

export class JournalActions {
  public constructor(private readonly labels: () => CatalogLabels) {}

  public classify(actions: readonly JournalAction[]): readonly JournalAction[] {
    const automatic = new Set(this.labels().automaticSkills ?? []);

    return actions.map((action) =>
      action.kind === JournalActionKind.Cast && automatic.has(action.skillId ?? 0)
        ? { ...action, kind: JournalActionKind.Trigger }
        : action,
    );
  }
}
