import type { JournalEffect } from './journal-effect';
import type { JournalResource } from './journal-resource';
import type { JournalAction } from './journal-action';
import type { JournalUnit } from './journal-unit';
import type { JournalCombat } from './journal-combat';
import type { JournalRecord } from './journal-record';
import type { JournalSourceStatus } from './journal-source-status';
export interface JournalFrame {
  readonly observationId: string;
  readonly atUs: number;
  readonly status: JournalSourceStatus;
  readonly error: string | null;
  readonly interrupted?: boolean;
  readonly playerId: number | null;
  readonly mapId: number | null;
  readonly dungeonId: number | null;
  readonly combat: JournalCombat | null;
  readonly units: readonly JournalUnit[];
  readonly effects?: readonly JournalEffect[];
  readonly resources?: readonly JournalResource[];
  readonly actions?: readonly JournalAction[];
  readonly firstEventSequence?: number | null;
  readonly firstEventAtUs?: number | null;
  readonly eventSequence?: number | null;
  readonly records: readonly JournalRecord[];
}
