export interface JournalCombat {
  readonly active: boolean | null;
  readonly startedAtUs: number | null;
  readonly elapsedMs: number | null;
  readonly encounter: { readonly npcId: number; readonly entityId: number } | null;
}
