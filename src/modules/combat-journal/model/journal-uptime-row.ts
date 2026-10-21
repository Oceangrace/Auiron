export interface JournalUptimeRow {
  readonly key: string;
  readonly skillId: number | null;
  readonly effectId: number | null;
  readonly milliseconds: number;
  readonly intervals: readonly (readonly [number, number])[];
}
