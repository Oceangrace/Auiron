import { combatStatisticsCatalog } from '../model/rules/combat-statistics-catalog';
import { FieldCategory } from './field-category';
import type { FormulaDefinition } from '../model/rules/formula-definition';
import type { TriggerRule } from '../model/rules/trigger-rule';

export class FieldLabels {
  private readonly combatFields = new Set(combatStatisticsCatalog.map((field) => field.path));
  public constructor(
    private readonly translate: (key: string, params?: Record<string, unknown>) => string,
  ) {}

  public label(
    path: string,
    formulas: readonly FormulaDefinition[] = [],
    rules: readonly TriggerRule[] = [],
  ): string {
    const ruleLabel = this.ruleLabel(path, rules);

    if (ruleLabel !== null) {
      return ruleLabel;
    }

    if (path.startsWith('formula.') === true) {
      return (
        this.translate('modules.interfaces.formulas.title') +
        ' · ' +
        (formulas.find((formula) => formula.id === path.slice(8))?.name ?? path.slice(8))
      );
    }

    const dot = path.indexOf('.');

    const scope = path.slice(0, dot);

    const field = path.slice(dot + 1).replace('.', '_');

    const label = field.startsWith('stat_')
      ? this.translate('modules.interfaces.fieldNames.stat', { id: field.slice(5) })
      : this.translate('modules.interfaces.fieldNames.' + field);

    return this.scope(scope) + ' · ' + label;
  }

  public scope(scope: string): string {
    if (scope === 'rule') {
      return this.translate('modules.interfaces.rules.title');
    }

    if (scope === 'formula') {
      return this.translate('modules.interfaces.formulas.title');
    }

    if (scope === 't1') {
      return this.translate('modules.interfaces.data.trigger');
    }

    return scope.startsWith('party') === true
      ? this.translate('modules.interfaces.data.party', { n: scope.slice(5) })
      : this.translate(`modules.interfaces.data.scopes.${scope}`);
  }

  public group(path: string): string {
    if (path.startsWith('formula.') === true) {
      return this.translate('modules.interfaces.formulas.title');
    }

    const dot = path.indexOf('.');

    const scope = path.slice(0, dot);

    if (scope.startsWith('party') === true) {
      return this.scope(scope);
    }

    if (scope === 'group') {
      return this.translate('modules.interfaces.fieldGroups.party');
    }

    if (['combat', 'zone', 'event', 't1', 'rule'].includes(scope) === true) {
      return this.scope(scope);
    }

    return this.translate('modules.interfaces.fieldGroups.' + this.category(path.slice(dot + 1)));
  }

  public parentGroup(path: string): string {
    const scope = path.slice(0, path.indexOf('.'));

    return scope.startsWith('party') === true ? this.scope('group') : this.scope(scope);
  }

  private ruleLabel(path: string, rules: readonly TriggerRule[]): string | null {
    const index = rules.findIndex((rule) => path.startsWith('rule.' + rule.id + '.'));

    const rule = rules[index];

    if (rule === undefined) {
      return null;
    }

    const field = path.slice(('rule.' + rule.id + '.').length);

    if (field !== 'active') {
      return (
        String(index + 1) +
        ' · ' +
        this.translate('modules.interfaces.rules.types.' + rule.kind) +
        ' · ' +
        this.translate('modules.interfaces.fieldNames.' + field)
      );
    }

    return this.translate('modules.interfaces.conditions.ruleActive', {
      n: index + 1,
      kind: this.translate('modules.interfaces.rules.types.' + rule.kind),
    });
  }

  private category(field: string): FieldCategory {
    if (field.startsWith('hp.') === true || field.startsWith('mp.') === true) {
      return FieldCategory.Health;
    }

    if (field.startsWith('stat.') === true) {
      return FieldCategory.Stats;
    }

    if (this.combatFields.has(field) === true) {
      return FieldCategory.Combat;
    }

    if (field.startsWith('npc') === true || ['isBoss', 'isDummy'].includes(field) === true) {
      return FieldCategory.Npc;
    }

    return [
      'id',
      'name',
      'class',
      'skillClass',
      'level',
      'serverId',
      'ownerId',
      'gearScore',
      'combatPower',
    ].includes(field)
      ? FieldCategory.Identity
      : FieldCategory.State;
  }
}
