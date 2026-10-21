export interface JournalEffectSpan {
  readonly key: string;
  readonly skillId: number | null;
  readonly effectId: number | null;
  readonly startMs: number;
  readonly endMs: number;
}
