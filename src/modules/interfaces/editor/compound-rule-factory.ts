import type { TriggerRule } from '../model/rules/trigger-rule';
import { TriggerType } from '../model/rules/trigger-type';
import { TriggerUnit } from '../model/rules/trigger-unit';
import { EffectFilter } from '../model/rules/effect-filter';
import { EffectOrigin } from '../model/rules/effect-origin';
import { EffectPresence } from '../model/rules/effect-presence';
import { CloneDirection } from '../model/rules/clone-direction';
import { GameEventType } from '../model/rules/game-event-type';
import { EventParticipant } from '../model/rules/event-participant';
import { ComparisonOperator } from '../model/rules/comparison-operator';
import { BossEvent } from '../model/rules/boss-event';
export class CompoundRuleFactory {
  public create(kind: TriggerType): TriggerRule {
    const base = { id: crypto.randomUUID(), observe: false };

    if (kind === TriggerType.Boss) {
      return {
        ...base,
        kind,
        trigger: {
          bossId: 111022,
          event: BossEvent.BeforeSpawn,
          beforeSeconds: 180,
          durationMs: 6000,
        },
      };
    }

    if (kind === TriggerType.Effect) {
      return {
        ...base,
        kind,
        trigger: {
          unit: TriggerUnit.Player,
          member: '',
          filter: EffectFilter.Any,
          effects: [],
          presence: EffectPresence.Present,
          origin: EffectOrigin.Any,
          minStacks: 1,
          remainingBelowSeconds: 0,
          clones: false,
          cloneDirection: CloneDirection.Right,
        },
      };
    }

    if (kind === TriggerType.Event) {
      return {
        ...base,
        kind,
        trigger: {
          event: GameEventType.SkillUsed,
          source: EventParticipant.Player,
          recipient: EventParticipant.Any,
          skillId: null,
          effectId: null,
          minValue: null,
          durationMs: 1500,
          delayMs: 0,
          schedule: null,
          sourceNpcId: null,
          useWidgetSkill: false,
        },
      };
    }

    return {
      ...base,
      kind: TriggerType.Value,
      trigger: { path: 'player.hp.value', operator: ComparisonOperator.Greater, value: '0' },
    };
  }
}
