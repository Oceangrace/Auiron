import type { TriggerRuleBase } from './trigger-rule-base';
import type { TriggerType } from './trigger-type';
import type { BossTrigger } from './boss-trigger';

export interface BossTriggerRule extends TriggerRuleBase {
  readonly kind: TriggerType.Boss;
  readonly trigger: BossTrigger;
}
