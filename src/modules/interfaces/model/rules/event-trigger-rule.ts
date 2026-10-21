import type { TriggerRuleBase } from './trigger-rule-base';
import type { TriggerType } from './trigger-type';
import type { EventTrigger } from './event-trigger';

export interface EventTriggerRule extends TriggerRuleBase {
  readonly kind: TriggerType.Event;
  readonly trigger: EventTrigger;
}
