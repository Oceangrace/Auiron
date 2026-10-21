import type { FormulaNodeKind } from './formula-node-kind';

export interface FormulaField {
  readonly kind: FormulaNodeKind.Field;
  readonly path: string;
}
