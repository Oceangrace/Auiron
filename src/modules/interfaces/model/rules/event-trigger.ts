import type { GameEventType } from './game-event-type';
import type { EventParticipant } from './event-participant';
import type { EncounterSchedule } from './encounter-schedule';

export interface EventTrigger {
  readonly schedule?: EncounterSchedule | null;
  readonly sourceActorKey?: string | null;
  readonly sourceNpcId?: number | null;
  readonly useWidgetSkill?: boolean;
  readonly event: GameEventType;
  readonly source: EventParticipant;
  readonly recipient: EventParticipant;
  readonly skillId: number | null;
  readonly effectId: number | null;
  readonly minValue: number | null;
  readonly durationMs: number;
  readonly delayMs?: number;
}
