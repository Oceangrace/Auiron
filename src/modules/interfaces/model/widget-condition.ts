import type { ConditionMode } from './condition-mode';
import type { ConditionCheck } from './condition-check';
import type { ConditionChange } from './condition-change';

export interface WidgetCondition {
  readonly id: string;
  readonly enabled: boolean;
  readonly mode: ConditionMode;
  readonly checks: readonly ConditionCheck[];
  readonly changes: readonly ConditionChange[];
}
