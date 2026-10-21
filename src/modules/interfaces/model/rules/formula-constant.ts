import type { FormulaNodeKind } from './formula-node-kind';

export interface FormulaConstant {
  readonly kind: FormulaNodeKind.Constant;
  readonly value: number | boolean;
}
