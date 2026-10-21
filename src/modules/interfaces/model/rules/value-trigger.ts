import type { ComparisonOperator } from './comparison-operator';

export interface ValueTrigger {
  readonly path: string;
  readonly operator: ComparisonOperator;
  readonly value: string;
}
