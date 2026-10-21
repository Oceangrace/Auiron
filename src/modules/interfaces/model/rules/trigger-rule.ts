import type { EffectTriggerRule } from './effect-trigger-rule';
import type { EventTriggerRule } from './event-trigger-rule';
import type { ValueTriggerRule } from './value-trigger-rule';

import type { BossTriggerRule } from './boss-trigger-rule';

export type TriggerRule = EffectTriggerRule | EventTriggerRule | ValueTriggerRule | BossTriggerRule;
