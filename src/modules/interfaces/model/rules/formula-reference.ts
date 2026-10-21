import type { FormulaNodeKind } from './formula-node-kind';

export interface FormulaReference {
  readonly kind: FormulaNodeKind.Formula;
  readonly formulaId: string;
}
