import type { FormulaNodeKind } from './formula-node-kind';
import type { FormulaOperation } from './formula-operation';
import type { FormulaConstant } from './formula-constant';
import type { FormulaField } from './formula-field';
import type { FormulaReference } from './formula-reference';

export interface FormulaOperationNode {
  readonly kind: FormulaNodeKind.Operation;
  readonly operation: FormulaOperation;
  readonly operands: readonly (
    FormulaConstant | FormulaField | FormulaReference | FormulaOperationNode
  )[];
}
