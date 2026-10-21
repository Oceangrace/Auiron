import type { CatalogField } from '../model/rules/catalog-field';
import type { FormulaDefinition } from '../model/rules/formula-definition';
import type { TriggerRule } from '../model/rules/trigger-rule';
import { DataKind } from '../model/rules/data-kind';
import { TriggerType } from '../model/rules/trigger-type';
import { gameCatalog, effectCatalog } from '../model/rules/game-catalog';
import { rowCatalog } from '../model/rules/row-catalog';
import { eventCatalog } from '../model/rules/event-catalog';

import { bossCatalog } from '../model/rules/boss-catalog';

export class FieldCatalog {
  public entries(
    formulas: readonly FormulaDefinition[] = [],
    rules: readonly TriggerRule[] = [],
  ): readonly CatalogField[] {
    return [
      ...gameCatalog,
      ...effectCatalog.map((field) => ({ ...field, path: 't1.' + field.path })),
      ...bossCatalog
        .filter((field) => !effectCatalog.some((entry) => entry.path === field.path))
        .map((field) => ({ ...field, path: 't1.' + field.path })),
      ...rules
        .filter((rule) => rule.kind === TriggerType.Boss)
        .flatMap((rule) =>
          bossCatalog.map((field) => ({ ...field, path: 'rule.' + rule.id + '.' + field.path })),
        ),
      ...rowCatalog,
      ...eventCatalog,
      ...formulas.map((formula) => ({ path: 'formula.' + formula.id, kind: formula.kind })),
      ...rules
        .filter((rule) => rule.kind !== TriggerType.Value)
        .map((rule) => ({ path: 'rule.' + rule.id + '.active', kind: DataKind.Boolean })),
    ];
  }

  public kind(
    path: string,
    formulas: readonly FormulaDefinition[] = [],
    rules: readonly TriggerRule[] = [],
  ): DataKind | null {
    return this.entries(formulas, rules).find((field) => field.path === path)?.kind ?? null;
  }
}
