import type { TriggerRuleBase } from './trigger-rule-base';
import type { TriggerType } from './trigger-type';
import type { ValueTrigger } from './value-trigger';

export interface ValueTriggerRule extends TriggerRuleBase {
  readonly kind: TriggerType.Value;
  readonly trigger: ValueTrigger;
}
