export interface JournalResource {
  readonly sequence: number;
  readonly atUs: number;
  readonly unitId: number;
  readonly health: number | null;
  readonly healthMax: number | null;
  readonly mana: number | null;
  readonly manaMax: number | null;
}
