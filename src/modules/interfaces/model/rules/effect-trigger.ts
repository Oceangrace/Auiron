import type { CloneDirection } from './clone-direction';
import type { EffectFilter } from './effect-filter';
import type { EffectOrigin } from './effect-origin';
import type { EffectPresence } from './effect-presence';
import type { TriggerUnit } from './trigger-unit';

export interface EffectTrigger {
  readonly unit: TriggerUnit;
  readonly member: string;
  readonly filter: EffectFilter;
  /** ID эффектов из справочника игры; пусто — любой эффект. */
  readonly effects: readonly number[];
  readonly presence: EffectPresence;
  readonly origin: EffectOrigin;
  readonly minStacks: number;
  /** Срабатывать, только когда осталось меньше N секунд; 0 — всегда. */
  readonly remainingBelowSeconds: number;
  readonly clones: boolean;
  readonly cloneDirection: CloneDirection;
}
