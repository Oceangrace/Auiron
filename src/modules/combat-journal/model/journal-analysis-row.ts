import type { JournalRow } from './journal-row';
export interface JournalAnalysisRow {
  readonly row: JournalRow;
  readonly amount: number;
  readonly hits: number;
  readonly average: number | null;
  readonly rate: number | null;
  readonly critical: number | null;
  readonly periodic: number;
}
