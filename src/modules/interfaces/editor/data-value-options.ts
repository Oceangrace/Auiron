import type { UiSelectOption } from '../../../renderer/components/ui/ui-select-option';
import type { FormulaDefinition } from '../model/rules/formula-definition';
import type { TriggerRule } from '../model/rules/trigger-rule';
import { FieldCatalog } from './field-catalog';
import { DataKind } from '../model/rules/data-kind';
import { EffectFilter } from '../model/rules/effect-filter';
import { GameEventType } from '../model/rules/game-event-type';
import { SourceStatus } from '../model/source-status';

export class DataValueOptions {
  public constructor(private readonly translate: (key: string) => string) {}

  public choices(
    path: string,
    formulas: readonly FormulaDefinition[],
    rules: readonly TriggerRule[],
  ): readonly UiSelectOption<string>[] {
    if (new FieldCatalog().kind(path, formulas, rules) === DataKind.Boolean) {
      return [
        { value: 'true', label: this.translate('modules.interfaces.conditions.yes') },
        { value: 'false', label: this.translate('modules.interfaces.conditions.no') },
      ];
    }

    if (
      path === 't1.kind' ||
      (path.startsWith('rule.') === true && path.endsWith('.kind') === true)
    ) {
      return [EffectFilter.Buff, EffectFilter.Debuff].map((value) => ({
        value,
        label: this.translate('modules.interfaces.rules.options.' + value),
      }));
    }

    if (path === 'source.status') {
      return Object.values(SourceStatus).map((value) => ({
        value,
        label: this.translate('modules.interfaces.status.' + value),
      }));
    }

    if (path === 'event.type') {
      return Object.values(GameEventType).map((value) => ({
        value,
        label: this.translate('modules.interfaces.rules.options.' + value),
      }));
    }

    return [];
  }
}
