export interface JournalEffect {
  readonly sequence: number;
  readonly atUs: number;
  readonly unitId: number;
  readonly instanceId: number;
  readonly effectId: number | null;
  readonly skillId: number | null;
  readonly sourceId: number | null;
  readonly durationMs: number | null;
  readonly expiresAtUs?: number | null;
  readonly removed: boolean;
  readonly removedBy?: number | null;
  readonly removalSkill?: number | null;
  readonly removalReason?: number | null;
}
