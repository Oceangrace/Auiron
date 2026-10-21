import type { LogicMode } from './logic-mode';
import type { TriggerRule } from './trigger-rule';

export interface TriggerGroup {
  readonly mode: LogicMode;
  readonly dataRuleId: string | null;
  readonly rules: readonly TriggerRule[];
}
