import type { ConditionField } from './condition-field';
import type { ComparisonOperator } from './comparison-operator';
export interface ConditionCheck {
  readonly path?: string | null;
  readonly textValue?: string | null;
  readonly field: ConditionField;
  readonly operator: ComparisonOperator;
  readonly value: number;
}
