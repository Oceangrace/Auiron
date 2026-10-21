import type { JournalEffect } from '../model/journal-effect';
import type { JournalResource } from '../model/journal-resource';
import type { JournalAction } from '../model/journal-action';
import type { JournalFrame } from '../model/journal-frame';
import type { JournalRecord } from '../model/journal-record';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalUnit } from '../model/journal-unit';
import { JournalPeriod } from '../model/journal-period';
import { JournalStatisticsDelta } from './journal-statistics-delta';
import { CombatCounter } from '../model/combat-counter';

export class JournalHistory {
  private static readonly TRACED_FIGHTS = 10;
  private readonly delta = new JournalStatisticsDelta();
  private readonly baseline = new Map<string, JournalRecord>();
  private observation = '';
  private current: JournalSegment | null = null;
  private previous: readonly JournalSegment[] = [];
  private total: JournalSegment | null = null;
  private run: JournalSegment | null = null;
  private lastRun: JournalSegment | null = null;
  private dungeon: number | null = null;
  private combatKey: number | null = null;
  private fault = false;
  private archiveRevision = 0;
  private lastSequence = 0;
  private sequenceOffset = 0;

  public list(): readonly JournalSegment[] {
    return [
      ...(this.current === null ? [] : [this.current]),
      ...(this.run === null ? [] : [this.run]),
      ...this.previous,
    ];
  }

  public read(period: JournalPeriod, selected: string | null): JournalSegment | null {
    if (selected !== null) {
      return this.list().find((entry) => entry.id === selected) ?? null;
    }

    switch (period) {
      case JournalPeriod.Run:
        return this.run ?? this.lastRun;
      case JournalPeriod.Total:
        return this.total;
      case JournalPeriod.Current:
        return this.latest();
    }
  }

  private latest(): JournalSegment | null {
    return (
      this.current ?? this.previous.find((entry) => entry.period !== JournalPeriod.Run) ?? null
    );
  }

  public clearHistory(): void {
    this.previous = [];
    this.archiveRevision++;
  }

  public restore(reports: readonly JournalSegment[], limit: number): void {
    this.previous = this.trim(reports.filter((report) => report.endedAtUs !== null)).slice(
      0,
      limit,
    );
    this.lastRun = this.previous.find((report) => report.period === JournalPeriod.Run) ?? null;
  }

  // A full trace is up to 24 000 events; keeping one per archived fight costs hundreds of
  // megabytes. Older fights keep totals and statistics, their event list is dropped.
  private trim(segments: readonly JournalSegment[]): readonly JournalSegment[] {
    return segments.map((segment, index) =>
      index < JournalHistory.TRACED_FIGHTS ||
      ((segment.actions?.length ?? 0) === 0 && (segment.effects?.length ?? 0) === 0)
        ? segment
        : { ...segment, actions: [], effects: [], tracePartial: true },
    );
  }

  public archived(): readonly JournalSegment[] {
    return this.previous;
  }

  public archiveVersion(): number {
    return this.archiveRevision;
  }

  public resetRun(): void {
    this.run = null;
    this.lastRun = null;
  }

  public update(frame: JournalFrame, limit: number): void {
    frame = this.alignSequences(frame);
    this.context(frame, limit);
    const changes = this.changes(frame.records);

    const active = frame.combat?.active === true;

    if (this.current === null && active === false && this.offensive(changes) === false) {
      this.lastSequence = frame.eventSequence ?? this.lastSequence;

      return;
    }

    const increment = this.updateCurrent(frame, changes);

    this.total = this.aggregate(this.total, frame, changes, increment);
    this.run = {
      ...this.aggregate(this.run, frame, changes, increment),
      period: JournalPeriod.Run,
      encounterId: null,
      npcId: null,
    };
    this.lastSequence = frame.eventSequence ?? this.lastSequence;

    if (active === false) {
      this.finish(limit);
    }
  }

  private updateCurrent(frame: JournalFrame, changes: readonly JournalRecord[]): number {
    const previous = this.current?.elapsedMs ?? 0;

    this.current = this.merge(this.current ?? this.empty(frame), frame, changes);
    const elapsed = Math.max(previous, frame.combat?.elapsedMs ?? this.current.elapsedMs);

    this.current = { ...this.current, elapsedMs: elapsed };

    return Math.max(0, elapsed - previous);
  }

  private offensive(records: readonly JournalRecord[]): boolean {
    return records.some(
      (record) =>
        (record.statistics.counters[CombatCounter.Hits] ?? 0) !== 0 ||
        (record.statistics.counters[CombatCounter.UnknownHits] ?? 0) !== 0,
    );
  }

  private aggregate(
    segment: JournalSegment | null,
    frame: JournalFrame,
    changes: readonly JournalRecord[],
    elapsed: number,
  ): JournalSegment {
    const merged = this.merge(segment ?? this.empty(frame), frame, changes);

    return {
      ...merged,
      elapsedMs: (segment?.elapsedMs ?? 0) + elapsed,
      spanMs: Math.max(0, (frame.atUs - merged.startedAtUs) / 1000),
    };
  }

  private context(frame: JournalFrame, limit: number): void {
    this.changeObservation(frame, limit);

    if (frame.error !== null) {
      this.fault = true;
    }

    // Matchmaking can retain the previous dungeon ID after returning to the world.
    const dungeon = frame.mapId ?? frame.dungeonId;

    if (dungeon !== this.dungeon && dungeon !== null) {
      this.finish(limit);
      this.finishRun(frame.atUs, limit);
      this.dungeon = dungeon;
    }

    this.changeCombat(frame.combat?.startedAtUs ?? null, limit);
  }

  private changeCombat(key: number | null, limit: number): void {
    if (key !== this.combatKey && this.combatKey !== null) {
      this.finish(limit);
    }

    if (key !== null && key !== this.combatKey) {
      this.baseline.clear();
    }

    this.combatKey = key;
  }

  private changeObservation(frame: JournalFrame, limit: number): void {
    if (this.observation === frame.observationId) {
      return;
    }

    const sameRun =
      this.run !== null &&
      frame.mapId !== null &&
      frame.mapId === this.dungeon &&
      frame.playerId !== null &&
      frame.playerId === this.run.playerId;

    this.markPartial(frame);
    this.finish(limit);
    this.baseline.clear();
    this.observation = frame.observationId;

    if (sameRun === false) {
      this.total = null;
      this.finishRun(frame.atUs, limit);
      this.dungeon = null;
      this.lastSequence = 0;
    }

    this.combatKey = null;
    this.fault = false;
  }

  private alignSequences(frame: JournalFrame): JournalFrame {
    this.updateSequenceOffset(frame);

    if (this.sequenceOffset === 0) {
      return frame;
    }

    return {
      ...frame,
      eventSequence: (frame.eventSequence ?? 0) + this.sequenceOffset,
      firstEventSequence: (frame.firstEventSequence ?? 0) + this.sequenceOffset,
      actions: (frame.actions ?? []).map((item) => ({
        ...item,
        sequence: item.sequence + this.sequenceOffset,
      })),
      effects: (frame.effects ?? []).map((item) => ({
        ...item,
        sequence: item.sequence + this.sequenceOffset,
      })),
      resources: (frame.resources ?? []).map((item) => ({
        ...item,
        sequence: item.sequence + this.sequenceOffset,
      })),
    };
  }

  private updateSequenceOffset(frame: JournalFrame): void {
    const restarted = (frame.eventSequence ?? 0) + this.sequenceOffset < this.lastSequence;

    if (this.observation !== frame.observationId && restarted === true) {
      this.sequenceOffset = this.lastSequence;
    }
  }

  private changes(records: readonly JournalRecord[]): readonly JournalRecord[] {
    const changes: JournalRecord[] = [];

    for (const record of records) {
      const key = this.key(record);

      const statistics = this.delta.between(record.statistics, this.baseline.get(key)?.statistics);

      this.baseline.set(key, record);

      if (this.delta.changed(statistics) === true) {
        changes.push({
          ...record,
          statistics: { ...statistics, complete: statistics.complete && this.fault === false },
        });
      }
    }

    return changes;
  }

  private merge(
    segment: JournalSegment,
    frame: JournalFrame,
    changes: readonly JournalRecord[],
  ): JournalSegment {
    const records = new Map(segment.records.map((record) => [this.key(record), record]));

    for (const record of changes) {
      const key = this.key(record);

      records.set(key, {
        ...record,
        statistics: this.delta.merge(records.get(key)?.statistics, record.statistics),
      });
    }

    const units = { ...segment.units };

    for (const unit of frame.units) {
      units[unit.id] = this.unit(units[unit.id], unit);
    }

    const resources = this.resources(segment, frame);

    return {
      ...segment,
      units,
      records: [...records.values()],
      actions: this.actions(segment, frame),
      effects: this.effects(segment, frame),
      resources: resources.slice(-12_000),
      resourcesPartial: segment.resourcesPartial === true || resources.length > 12_000,
      tracePartial: this.tracePartial(segment, frame),
      ...this.encounter(frame, segment),
      playerId: frame.playerId ?? segment.playerId,
    };
  }

  private tracePartial(segment: JournalSegment, frame: JournalFrame): boolean {
    const first = frame.firstEventSequence ?? 0;

    const gap = this.lastSequence !== 0 && first > this.lastSequence + 1;

    const late = this.lateTrace(segment, frame);

    return (
      segment.tracePartial === true || this.fault || gap || late || this.traceLimit(segment, frame)
    );
  }

  private lateTrace(segment: JournalSegment, frame: JournalFrame): boolean {
    return this.lastSequence === 0 && (frame.firstEventAtUs ?? 0) > segment.startedAtUs + 500_000;
  }

  private traceLimit(segment: JournalSegment, frame: JournalFrame): boolean {
    return (
      this.actions(segment, frame).length >= 12_000 || this.effects(segment, frame).length >= 12_000
    );
  }

  private effects(segment: JournalSegment, frame: JournalFrame): readonly JournalEffect[] {
    const old = segment.effects ?? [];

    const last = old.at(-1)?.sequence ?? this.lastSequence;

    const fresh = (frame.effects ?? []).filter(
      (effect) => effect.sequence > last && effect.atUs >= segment.startedAtUs,
    );

    return [...old, ...fresh].slice(-12_000);
  }

  private actions(segment: JournalSegment, frame: JournalFrame): readonly JournalAction[] {
    const old = segment.actions ?? [];

    const last = old.at(-1)?.sequence ?? this.lastSequence;

    const fresh = (frame.actions ?? []).filter(
      (action) =>
        action.sequence > last && action.atUs >= segment.startedAtUs && action.atUs <= frame.atUs,
    );

    return [...old, ...fresh].slice(-12_000);
  }

  private unit(previous: JournalUnit | undefined, unit: JournalUnit): JournalUnit {
    if (previous === undefined) {
      return unit;
    }

    return {
      ...unit,
      name: unit.name ?? previous.name,
      className: unit.className ?? previous.className,
      group: unit.group === true || previous.group === true ? true : (unit.group ?? previous.group),
      player: unit.player || previous.player,
    };
  }

  private resources(segment: JournalSegment, frame: JournalFrame): readonly JournalResource[] {
    const old = segment.resources ?? [];

    const last = old.at(-1)?.sequence ?? this.lastSequence;

    const fresh = (frame.resources ?? []).filter(
      (event) =>
        event.sequence > last && event.atUs >= segment.startedAtUs && event.atUs <= frame.atUs,
    );

    return [...old, ...fresh];
  }

  private empty(frame: JournalFrame): JournalSegment {
    const start = frame.combat?.startedAtUs ?? frame.atUs;

    return {
      id: crypto.randomUUID(),
      startedAtUs: start,
      endedAtUs: null,
      elapsedMs: 0,
      mapId: frame.mapId,
      dungeonId: frame.dungeonId,
      encounterId: this.encounter(frame).encounterId,
      npcId: frame.combat?.encounter?.npcId ?? null,
      playerId: frame.playerId,
      units: {},
      records: [],
    };
  }

  private encounter(
    frame: JournalFrame,
    segment?: JournalSegment,
  ): Pick<Required<JournalSegment>, 'npcId' | 'encounterId'> {
    const current = frame.combat?.encounter;

    if (current !== null && current !== undefined) {
      return { npcId: current.npcId, encounterId: current.entityId };
    }

    return {
      npcId: segment?.npcId ?? null,
      encounterId: segment?.encounterId ?? null,
    };
  }

  private finish(limit: number): void {
    if (this.current === null) {
      return;
    }

    this.previous = this.trim([
      { ...this.current, endedAtUs: this.current.startedAtUs + this.current.elapsedMs * 1000 },
      ...this.previous,
    ]).slice(0, limit);
    this.current = null;
    this.archiveRevision++;
  }

  private finishRun(atUs: number, limit: number): void {
    if (this.run === null) {
      return;
    }

    this.lastRun = { ...this.run, endedAtUs: atUs };
    this.previous = this.trim([this.lastRun, ...this.previous]).slice(0, limit);
    this.run = null;
    this.archiveRevision++;
  }

  // A gap ends the observation mid-fight: the fight so far is archived as incomplete.
  private markPartial(frame: JournalFrame): void {
    if (frame.interrupted !== true && frame.error === null) {
      return;
    }

    const partial = (segment: JournalSegment | null): JournalSegment | null =>
      segment === null
        ? null
        : {
            ...segment,
            tracePartial: true,
            records: segment.records.map((record) => ({
              ...record,
              statistics: { ...record.statistics, complete: false },
            })),
          };

    this.current = partial(this.current);
    this.run = partial(this.run);
  }

  private key(record: JournalRecord): string {
    return [record.sourceId, record.recipientId, record.skillId].join(':');
  }
}
