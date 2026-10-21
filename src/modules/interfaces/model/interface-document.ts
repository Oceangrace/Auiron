import type { InterfaceDisplay } from './interface-display';
import type { InterfaceLocalization } from './interface-localization';
import type { FormulaDefinition } from './rules/formula-definition';
import type { WidgetNode } from './widget-node';

export interface InterfaceDocument {
  readonly imported?: boolean;
  readonly localization?: InterfaceLocalization;
  readonly display?: InterfaceDisplay;
  readonly formulas: readonly FormulaDefinition[];
  readonly id: string;
  readonly name: string;
  readonly author: string;
  readonly version: string;
  readonly description: string;
  readonly enabled: boolean;
  readonly roots: readonly WidgetNode[];
}
