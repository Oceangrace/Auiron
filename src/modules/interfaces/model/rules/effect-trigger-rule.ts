import type { TriggerRuleBase } from './trigger-rule-base';
import type { TriggerType } from './trigger-type';
import type { EffectTrigger } from './effect-trigger';

export interface EffectTriggerRule extends TriggerRuleBase {
  readonly kind: TriggerType.Effect;
  readonly trigger: EffectTrigger;
}
