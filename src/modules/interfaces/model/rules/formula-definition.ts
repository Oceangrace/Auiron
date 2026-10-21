import type { DataKind } from './data-kind';
import type { FormulaNode } from './formula-node';

export interface FormulaDefinition {
  readonly id: string;
  readonly name: string;
  readonly kind: DataKind.Number | DataKind.Boolean | DataKind.Percent | DataKind.Duration;
  readonly expression: FormulaNode;
}
