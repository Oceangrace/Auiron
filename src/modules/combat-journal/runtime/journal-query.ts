import type { JournalWindow } from '../model/journal-window';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalRecord } from '../model/journal-record';
import type { JournalRow } from '../model/journal-row';
import { JournalDetail } from '../model/journal-detail';
import { JournalScope } from '../model/journal-scope';
import { JournalMetrics } from './journal-metrics';
import { JournalStatisticsDelta } from './journal-statistics-delta';

export class JournalQuery {
  private readonly metrics = new JournalMetrics();
  private readonly delta = new JournalStatisticsDelta();
  public constructor(
    private readonly skillName: (id: number | null) => string,
    private readonly unitName: (id: number, segment: JournalSegment) => string,
  ) {}

  public skill(id: number | null): string {
    return this.skillName(id);
  }

  public participants(segment: JournalSegment, scope: JournalScope): readonly number[] {
    return Object.values(segment.units)
      .map((unit) => unit.id)
      .filter((id) => this.accept(id, segment, scope));
  }

  public rows(
    segment: JournalSegment | null,
    window: JournalWindow,
    detail = JournalDetail.Actors,
    actor: number | null = null,
  ): readonly JournalRow[] {
    if (segment === null) {
      return [];
    }

    const groups = new Map<string, JournalRow>();

    for (const record of segment.records.filter((entry) =>
      this.metrics.relevant(entry.statistics, window.metric),
    )) {
      this.add(groups, record, segment, window, detail, actor);
    }

    const sorted = [...groups.values()]
      .filter((row) => row.knownValue !== 0 || row.statistics.complete === false)
      .sort((a, b) =>
        b.knownValue === a.knownValue
          ? a.label.localeCompare(b.label)
          : b.knownValue - a.knownValue,
      );

    const total = sorted.reduce((sum, row) => sum + row.knownValue, 0);

    const lead = sorted[0]?.knownValue ?? 0;

    const duplicates = this.duplicates(sorted);

    return sorted.slice(0, window.limit).map((row) => ({
      ...row,
      label: this.distinctLabel(row, duplicates),
      share: total === 0 ? 0 : (row.knownValue * 100) / total,
      bar: lead === 0 ? 0 : (row.knownValue * 100) / lead,
    }));
  }

  private duplicates(rows: readonly JournalRow[]): ReadonlyMap<string, string[]> {
    const groups = new Map<string, string[]>();

    for (const row of rows) {
      const ids = groups.get(row.label) ?? [];

      ids.push(row.id);
      groups.set(row.label, ids);
    }

    return groups;
  }

  private distinctLabel(row: JournalRow, duplicates: ReadonlyMap<string, string[]>): string {
    const ids = duplicates.get(row.label) ?? [];

    if (ids.length <= 1 === true) {
      return row.label;
    }

    const index = [...ids]
      .sort((a, b) => a.localeCompare(b, undefined, { numeric: true }))
      .indexOf(row.id);

    return row.label + ' · ' + String(index + 1);
  }

  private add(
    groups: Map<string, JournalRow>,
    record: JournalRecord,
    segment: JournalSegment,
    window: JournalWindow,
    detail: JournalDetail,
    actor: number | null,
  ): void {
    const own = this.metrics.incoming(window.metric) ? record.recipientId : record.sourceId;

    const root = window.mergeSummons ? this.root(own, segment) : own;

    if (this.accept(root, segment, window.scope) === false || (actor !== null && root !== actor)) {
      return;
    }

    const id = this.id(record, root, window, detail);

    const old = groups.get(id);

    const statistics = this.delta.merge(old?.statistics, record.statistics);

    const label = this.label(record, root, segment, window, detail);

    groups.set(id, {
      id,
      label,
      actorId: root,
      unit: segment.units[root] ?? null,
      statistics,
      value: this.metrics.value(statistics, window.metric, segment.elapsedMs),
      knownValue: this.metrics.known(statistics, window.metric, segment.elapsedMs),
      share: 0,
      bar: 0,
    });
  }

  private id(
    record: JournalRecord,
    actor: number,
    window: JournalWindow,
    detail: JournalDetail,
  ): string {
    if (detail === JournalDetail.Skills) {
      return String(record.skillId ?? 'unknown');
    }

    if (detail === JournalDetail.Targets) {
      return String(this.metrics.incoming(window.metric) ? record.sourceId : record.recipientId);
    }

    return String(actor);
  }

  private label(
    record: JournalRecord,
    actor: number,
    segment: JournalSegment,
    window: JournalWindow,
    detail: JournalDetail,
  ): string {
    if (detail === JournalDetail.Skills) {
      return this.skillName(record.skillId);
    }

    if (detail === JournalDetail.Targets) {
      return this.unitName(
        this.metrics.incoming(window.metric) ? record.sourceId : record.recipientId,
        segment,
      );
    }

    return this.unitName(actor, segment);
  }

  private root(id: number, segment: JournalSegment): number {
    const seen = new Set<number>();

    while (segment.units[id]?.ownerId !== null && segment.units[id]?.ownerId !== undefined) {
      if (seen.has(id) === true) {
        return id;
      }

      seen.add(id);
      id = segment.units[id]?.ownerId ?? id;
    }

    return id;
  }

  private accept(id: number, segment: JournalSegment, scope: JournalScope): boolean {
    switch (scope) {
      case JournalScope.All:
        return true;
      case JournalScope.Player:
        return id === segment.playerId;
      case JournalScope.Group:
        return id === segment.playerId || segment.units[id]?.group === true;
      case JournalScope.Players:
        return segment.units[id]?.player === true;
    }
  }
}
