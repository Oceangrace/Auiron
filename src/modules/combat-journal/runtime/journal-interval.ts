import type { JournalAction } from '../model/journal-action';
import type { JournalIntervalRow } from '../model/journal-interval-row';
import { JournalActionKind } from '../model/journal-action-kind';

export class JournalInterval {
  public rows(actions: readonly JournalAction[]): readonly JournalIntervalRow[] {
    const rows = new Map<number | null, JournalIntervalRow>();

    for (const action of actions) {
      if (
        [JournalActionKind.Damage, JournalActionKind.Healing, JournalActionKind.Cast].includes(
          action.kind,
        ) === false
      ) {
        continue;
      }

      const old = rows.get(action.skillId) ?? {
        skillId: action.skillId,
        damage: 0,
        healing: 0,
        hits: 0,
        casts: 0,
      };

      rows.set(action.skillId, {
        skillId: action.skillId,
        damage: old.damage + (action.kind === JournalActionKind.Damage ? action.value : 0),
        healing: old.healing + (action.kind === JournalActionKind.Healing ? action.value : 0),
        hits: old.hits + (action.kind === JournalActionKind.Cast ? 0 : 1),
        casts: old.casts + (action.kind === JournalActionKind.Cast ? 1 : 0),
      });
    }

    return [...rows.values()].sort((a, b) => b.damage + b.healing - a.damage - a.healing);
  }
}
