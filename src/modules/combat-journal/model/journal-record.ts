import type { JournalStatistics } from './journal-statistics';
export interface JournalRecord {
  readonly sourceId: number;
  readonly recipientId: number;
  readonly skillId: number | null;
  readonly statistics: JournalStatistics;
}
