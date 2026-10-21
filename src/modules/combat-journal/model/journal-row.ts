import type { JournalStatistics } from './journal-statistics';
import type { JournalUnit } from './journal-unit';
export interface JournalRow {
  readonly id: string;
  readonly label: string;
  readonly actorId: number | null;
  readonly unit: JournalUnit | null;
  readonly statistics: JournalStatistics;
  readonly value: number | null;
  readonly knownValue: number;
  readonly share: number;
  readonly bar: number;
}
