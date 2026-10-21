import type { CombatCounter } from './combat-counter';
export interface JournalStatistics {
  readonly counters: Readonly<Partial<Record<CombatCounter, number>>>;
  readonly firstAtUs: number | null;
  readonly lastAtUs: number | null;
  readonly minimum: number | null;
  readonly maximum: number | null;
  readonly complete: boolean;
}
