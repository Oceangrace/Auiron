import type { JournalStatistics } from '../model/journal-statistics';
import { CombatCounter } from '../model/combat-counter';
import { JournalMetric } from '../model/journal-metric';

export class JournalMetrics {
  private readonly damageMetrics = new Set([
    JournalMetric.Damage,
    JournalMetric.Dps,
    JournalMetric.DamageTaken,
    JournalMetric.Critical,
  ]);

  private readonly counters = new Map<JournalMetric, CombatCounter>([
    [JournalMetric.Damage, CombatCounter.Damage],
    [JournalMetric.Dps, CombatCounter.Damage],
    [JournalMetric.Healing, CombatCounter.Healing],
    [JournalMetric.Hps, CombatCounter.Healing],
    [JournalMetric.DamageTaken, CombatCounter.Damage],
    [JournalMetric.HealingTaken, CombatCounter.Healing],
    [JournalMetric.Hits, CombatCounter.Hits],
    [JournalMetric.Critical, CombatCounter.CriticalHits],
    [JournalMetric.Deaths, CombatCounter.Deaths],
    [JournalMetric.Misses, CombatCounter.Misses],
    [JournalMetric.Resists, CombatCounter.Resists],
  ]);

  public incoming(metric: JournalMetric): boolean {
    return metric === JournalMetric.DamageTaken || metric === JournalMetric.HealingTaken;
  }

  public relevant(statistics: JournalStatistics, metric: JournalMetric): boolean {
    const counter = this.counters.get(metric);

    if (this.count(statistics, counter) !== 0) {
      return true;
    }

    if (metric === JournalMetric.Critical) {
      return (
        this.count(statistics, CombatCounter.Hits) !== 0 ||
        this.count(statistics, CombatCounter.UnknownHits) !== 0
      );
    }

    return (
      this.damageMetrics.has(metric) && this.count(statistics, CombatCounter.UnknownHits) !== 0
    );
  }

  private count(statistics: JournalStatistics, counter: CombatCounter | undefined): number {
    return counter === undefined ? 0 : (statistics.counters[counter] ?? 0);
  }

  public known(statistics: JournalStatistics, metric: JournalMetric, duration: number): number {
    const counter = this.counters.get(metric);

    const value = counter === undefined ? 0 : (statistics.counters[counter] ?? 0);

    if (metric === JournalMetric.Dps || metric === JournalMetric.Hps) {
      return duration > 0 ? value / (duration / 1000) : 0;
    }

    if (metric === JournalMetric.Critical) {
      return (value * 100) / Math.max(1, statistics.counters[CombatCounter.DirectHits] ?? 0);
    }

    return value;
  }

  public value(
    statistics: JournalStatistics,
    metric: JournalMetric,
    duration: number,
  ): number | null {
    if (statistics.complete === false) {
      return null;
    }

    if ((metric === JournalMetric.Dps || metric === JournalMetric.Hps) && duration <= 0 === true) {
      return null;
    }

    return this.known(statistics, metric, duration);
  }
}
