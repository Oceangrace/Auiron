import type { JournalRow } from '../model/journal-row';
import type { JournalAnalysisRow } from '../model/journal-analysis-row';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalAction } from '../model/journal-action';
import { JournalAnalysisSort } from '../model/journal-analysis-sort';
import type { JournalActionKind } from '../model/journal-action-kind';
import { JournalMetric } from '../model/journal-metric';
import { CombatCounter } from '../model/combat-counter';

export class JournalAnalysis {
  public healing(metric: JournalMetric): boolean {
    return (
      metric === JournalMetric.Healing ||
      metric === JournalMetric.Hps ||
      metric === JournalMetric.HealingTaken
    );
  }

  public row(row: JournalRow, metric: JournalMetric, duration: number): JournalAnalysisRow {
    const healing = this.healing(metric);

    const amount = this.count(row, healing ? CombatCounter.Healing : CombatCounter.Damage);

    const hits = this.count(row, healing ? CombatCounter.HealHits : CombatCounter.Hits);

    return {
      row,
      amount,
      hits,
      average: row.statistics.complete ? this.average(amount, hits) : null,
      rate: duration === 0 ? null : amount / (duration / 1000),
      critical: this.ratio(row, CombatCounter.CriticalHits),
      periodic: this.count(
        row,
        healing ? CombatCounter.PeriodicHealing : CombatCounter.PeriodicDamage,
      ),
    };
  }

  private count(row: JournalRow, counter: CombatCounter): number {
    return row.statistics.counters[counter] ?? 0;
  }

  private average(amount: number, hits: number): number | null {
    return hits === 0 ? null : amount / hits;
  }

  public ratio(row: JournalRow, counter: CombatCounter): number | null {
    const hits = row.statistics.counters[CombatCounter.DirectHits] ?? 0;

    return hits === 0 || row.statistics.complete === false
      ? null
      : ((row.statistics.counters[counter] ?? 0) * 100) / hits;
  }

  public ordered(
    rows: readonly JournalAnalysisRow[],
    sort: JournalAnalysisSort,
  ): readonly JournalAnalysisRow[] {
    return [...rows].sort((a, b) => {
      if (sort === JournalAnalysisSort.Name) {
        return a.row.label.localeCompare(b.row.label);
      }

      const difference = this.order(b, sort) - this.order(a, sort);

      return difference === 0 ? a.row.label.localeCompare(b.row.label) : difference;
    });
  }

  private order(row: JournalAnalysisRow, sort: JournalAnalysisSort): number {
    switch (sort) {
      case JournalAnalysisSort.Hits:
        return row.hits;
      case JournalAnalysisSort.Average:
        return row.average ?? -1;
      case JournalAnalysisSort.Critical:
        return row.critical ?? -1;
      case JournalAnalysisSort.Amount:
      case JournalAnalysisSort.Name:
        return row.amount;
    }
  }

  public difference(
    current: JournalAnalysisRow | undefined,
    previous: JournalAnalysisRow | undefined,
  ): number | null {
    if (current?.row.value === null || previous?.row.value === null) {
      return null;
    }

    return this.rate(current) - this.rate(previous);
  }

  private rate(row: JournalAnalysisRow | undefined): number {
    return row?.rate ?? 0;
  }

  public root(id: number, segment: JournalSegment, merge: boolean): number {
    const seen = new Set<number>();

    while (merge === true) {
      const owner = segment.units[id]?.ownerId;

      if (owner === null || owner === undefined || seen.has(id) === true) {
        break;
      }

      seen.add(id);
      id = owner;
    }

    return id;
  }

  public exportRow(entry: JournalAnalysisRow): readonly (string | number)[] {
    return [
      entry.row.label,
      entry.amount,
      entry.row.share,
      entry.rate ?? '',
      entry.hits,
      entry.average ?? '',
      entry.critical ?? '',
      entry.periodic,
      String(entry.row.statistics.complete),
    ];
  }

  public actions(
    segment: JournalSegment,
    actor: number,
    merge: boolean,
    incoming: boolean,
    target: number | null,
  ): readonly JournalAction[] {
    return (segment.actions ?? []).filter((action) => {
      const owner = incoming ? action.recipientId : action.sourceId;

      const other = incoming ? action.sourceId : action.recipientId;

      return (
        owner !== null &&
        this.root(owner, segment, merge) === actor &&
        (target === null || target === other)
      );
    });
  }

  public chart(
    actions: readonly JournalAction[],
    kind: JournalActionKind,
    duration: number,
    startedAtUs: number,
  ): readonly number[] {
    if (duration <= 0 === true) {
      return [0];
    }

    const count = Math.max(1, Math.min(120, Math.ceil(duration / 1000)));

    const width = duration > 0 ? duration / count : 1000;

    const buckets = Array<number>(count).fill(0);

    const start = startedAtUs;

    for (const action of actions.filter(
      (entry) =>
        entry.kind === kind && entry.atUs >= start && entry.atUs <= start + duration * 1000,
    )) {
      const index = Math.max(
        0,
        Math.min(count - 1, Math.floor((action.atUs - start) / 1000 / width)),
      );

      buckets[index] = (buckets[index] ?? 0) + action.value / (width / 1000);
    }

    return buckets;
  }

  public csv(rows: readonly (readonly (string | number)[])[]): string {
    return (
      '\uFEFF' + rows.map((row) => row.map((value) => this.csvCell(value)).join(',')).join('\r\n')
    );
  }

  private csvCell(value: string | number): string {
    const text =
      typeof value === 'string' && /^[\s]*[=+\-@]/u.test(value) ? "'" + value : String(value);

    return '"' + text.replaceAll('"', '""') + '"';
  }
}
