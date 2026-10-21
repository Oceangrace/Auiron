import type { JournalClass } from './journal-class';
export interface JournalUnit {
  readonly id: number;
  readonly ownerId: number | null;
  readonly name: string | null;
  readonly className: JournalClass | null;
  readonly npcId: number | null;
  readonly player: boolean;
  readonly group: boolean | null;
}
