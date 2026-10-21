import type { JournalEffect } from './journal-effect';
import type { JournalResource } from './journal-resource';
import type { JournalAction } from './journal-action';
import type { JournalUnit } from './journal-unit';
import type { JournalRecord } from './journal-record';
import type { JournalPeriod } from './journal-period';
export interface JournalSegment {
  readonly id: string;
  readonly period?: JournalPeriod;
  readonly startedAtUs: number;
  readonly endedAtUs: number | null;
  readonly elapsedMs: number;
  readonly spanMs?: number;
  readonly mapId: number | null;
  readonly dungeonId?: number | null;
  readonly encounterId?: number | null;
  readonly npcId: number | null;
  readonly playerId: number | null;
  readonly units: Readonly<Record<number, JournalUnit>>;
  readonly effects?: readonly JournalEffect[];
  readonly resources?: readonly JournalResource[];
  readonly resourcesPartial?: boolean;
  readonly actions?: readonly JournalAction[];
  readonly tracePartial?: boolean;
  readonly records: readonly JournalRecord[];
}
