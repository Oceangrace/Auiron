import type { JournalStatistics } from '../model/journal-statistics';
import { CombatCounter } from '../model/combat-counter';

export class JournalStatisticsDelta {
  public between(current: JournalStatistics, previous?: JournalStatistics): JournalStatistics {
    const counters = Object.fromEntries(
      Object.values(CombatCounter).map((key) => [
        key,
        Math.max(0, (current.counters[key] ?? 0) - (previous?.counters[key] ?? 0)),
      ]),
    );

    return {
      counters,
      firstAtUs: previous?.lastAtUs ?? current.firstAtUs,
      lastAtUs: current.lastAtUs,
      minimum: current.minimum,
      maximum: current.maximum,
      complete: current.complete,
    };
  }

  public changed(value: JournalStatistics): boolean {
    return Object.values(value.counters).some((count) => count !== 0);
  }

  public merge(left: JournalStatistics | undefined, right: JournalStatistics): JournalStatistics {
    if (left === undefined) {
      return right;
    }

    const counters = Object.fromEntries(
      Object.values(CombatCounter).map((key) => [
        key,
        (left.counters[key] ?? 0) + (right.counters[key] ?? 0),
      ]),
    );

    return {
      counters,
      firstAtUs: this.minimum(left.firstAtUs, right.firstAtUs),
      lastAtUs: this.maximum(left.lastAtUs, right.lastAtUs),
      minimum: this.minimum(left.minimum, right.minimum),
      maximum: this.maximum(left.maximum, right.maximum),
      complete: left.complete && right.complete,
    };
  }

  private minimum(a: number | null, b: number | null): number | null {
    return a === null ? b : Math.min(a, b ?? a);
  }

  private maximum(a: number | null, b: number | null): number | null {
    return a === null ? b : Math.max(a, b ?? a);
  }
}
