import type { FormulaConstant } from './formula-constant';
import type { FormulaField } from './formula-field';
import type { FormulaReference } from './formula-reference';
import type { FormulaOperationNode } from './formula-operation-node';

export type FormulaNode = FormulaConstant | FormulaField | FormulaReference | FormulaOperationNode;
