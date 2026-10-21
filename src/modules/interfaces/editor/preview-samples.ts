import type { InterfaceWorkspace } from '../model/interface-workspace';
import type { PreviewContext } from '../model/preview-context';
import type { WidgetNode } from '../model/widget-node';
import { PreviewScenario } from '../model/preview-scenario';
import { TriggerType } from '../model/rules/trigger-type';
import { EffectFilter } from '../model/rules/effect-filter';
import { GroupDataSource } from '../model/rules/group-data-source';
import { WidgetKind } from '../model/widget-kind';
import { WidgetTree } from './widget-tree';
import { GameClass } from '../model/game-class';
import { PreviewCombat } from './preview-combat';

export class PreviewSamples {
  private readonly combat = new PreviewCombat();
  private readonly names = ['Aster', 'Mira', 'Orion', 'Luna', 'Aurion'];
  private readonly health = new Map<PreviewScenario, readonly number[]>([
    [PreviewScenario.Full, [100, 100, 100, 100, 100]],
    [PreviewScenario.Combat, [88, 100, 65, 42, 91]],
    [PreviewScenario.Critical, [15, 38, 0, 22, 67]],
  ]);

  public context(
    workspace: InterfaceWorkspace,
    scenario: PreviewScenario,
    members: number,
    elapsed = 0,
  ): PreviewContext {
    const nodes = new WidgetTree().flatten(workspace.documents.flatMap((entry) => entry.roots));

    const fields = {
      ...Object.fromEntries(
        nodes
          .filter((node) => node.appearance.iconPath !== null)
          .slice(0, 8)
          .map((node, index) => ['preview.icon.' + String(index), node.appearance.iconPath]),
      ),
      ...this.unit('player', 0, scenario, elapsed),
      ...this.unit('target', 2, scenario, elapsed),
      ...this.unit('targetTarget', 1, scenario, elapsed),
      ...this.unit('entity', 0, scenario, elapsed),
      'player.class':
        nodes.find((node) => node.load.playerClass !== '')?.load.playerClass ?? 'Cleric',
      'combat.inCombat':
        scenario === PreviewScenario.Combat || scenario === PreviewScenario.Critical,
      'combat.duration': elapsed * 1000,
      'combat.elapsed': elapsed * 1000,
      'preview.elapsed': elapsed,
      'preview.combat':
        scenario === PreviewScenario.Combat || scenario === PreviewScenario.Critical,
      'group.size': members,
      'group.active': members > 1,
    };

    const widgets = Object.fromEntries(
      nodes.map((node, index) => [node.id, this.trigger(node, scenario, elapsed, index % 5)]),
    );

    const instances = Object.fromEntries(
      nodes
        .filter((node) => this.clones(node))
        .map((node) => [node.id, this.effects(node, scenario, elapsed)]),
    );

    return {
      active: true,
      remaining: null,
      stacks: null,
      progress: null,
      fields,
      widgets,
      instances,
      rows: Array.from({ length: members }, (_, index) => ({
        ...this.unit('entity', index, scenario, elapsed),
        'row.name': this.names[index] ?? 'Aurion',
        'row.class': this.characterClass(index),
        'row.id': String(index + 1),
        'row.rank': index + 1,
        'row.value': Math.round((5 - index) * 12400 * (1 + elapsed / 20)),
        'row.percent': (5 - index) * 6.67,
        'row.leaderPercent': (5 - index) * 20,
      })),
    };
  }

  private unit(
    prefix: string,
    index: number,
    scenario: PreviewScenario,
    elapsed: number,
  ): Record<string, string | number | boolean | null> {
    const moving = this.combat.running(scenario);

    const percent = this.percent(scenario, elapsed, index);

    const maximum = 18000 + index * 2400;

    const mana = percent === null ? null : Math.max(0, percent - index * 9);

    return {
      [prefix + '.id']: String(index + 1),
      [prefix + '.name']: this.names[index] ?? 'Aurion',
      [prefix + '.exists']: true,
      [prefix + '.present']: true,
      [prefix + '.dead']: percent === null ? null : percent === 0,
      [prefix + '.level']: 45,
      [prefix + '.class']: this.characterClass(index),
      ...this.resource(prefix + '.hp', maximum, percent),
      ...this.resource(prefix + '.mp', 8000, mana),
      [prefix + '.hp.healing']:
        moving && percent !== 0 ? this.combat.healing(maximum, elapsed, index) : 0,
      ...this.cast(prefix, moving, elapsed),
    };
  }

  private percent(scenario: PreviewScenario, elapsed: number, index: number): number | null {
    const base = this.health.get(scenario)?.[index] ?? null;

    return this.combat.running(scenario) ? this.combat.percent(base, elapsed, index) : base;
  }

  private cast(prefix: string, moving: boolean, elapsed: number): Record<string, number | null> {
    return {
      [prefix + '.cast.remaining']: moving ? Math.max(0, 3000 - ((elapsed * 1000) % 5000)) : null,
      [prefix + '.cast.duration']: moving ? 3000 : null,
    };
  }

  private characterClass(index: number): GameClass {
    return Object.values(GameClass)[index % 8] ?? GameClass.Cleric;
  }

  private resource(
    prefix: string,
    maximum: number,
    percent: number | null,
  ): Record<string, number | null> {
    return {
      [prefix + '.value']: percent === null ? null : Math.round((maximum * percent) / 100),
      [prefix + '.max']: percent === null ? null : maximum,
      [prefix + '.percent']: percent,
    };
  }

  private trigger(
    node: WidgetNode,
    scenario: PreviewScenario,
    elapsed: number,
    index = 0,
  ): Record<string, string | number | boolean | null> {
    const data = { ...this.combat.timing(scenario, elapsed, index), name: node.name };

    const prefixes = ['t1', 'event', ...(node.rules?.rules.map((rule) => 'rule.' + rule.id) ?? [])];

    return Object.fromEntries(
      prefixes.flatMap((prefix) =>
        Object.entries(data).map(([key, value]) => [prefix + '.' + key, value]),
      ),
    );
  }

  private clones(node: WidgetNode): boolean {
    return (
      node.binding?.source === GroupDataSource.Effects ||
      node.rules?.rules.some((rule) => rule.kind === TriggerType.Effect && rule.trigger.clones) ===
        true
    );
  }

  private effects(
    node: WidgetNode,
    scenario: PreviewScenario,
    elapsed: number,
  ): readonly Readonly<Record<string, string | number | boolean | null>>[] {
    if (scenario === PreviewScenario.Full || scenario === PreviewScenario.Unknown) {
      return [];
    }

    return Array.from({ length: 3 }, (_, index) => this.trigger(node, scenario, elapsed, index));
  }

  public workspace(workspace: InterfaceWorkspace): InterfaceWorkspace {
    return {
      ...workspace,
      documents: workspace.documents.map((entry) => ({
        ...entry,
        roots: entry.roots.map((node) => this.node(node)),
      })),
    };
  }

  private node(node: WidgetNode): WidgetNode {
    const debuff = node.rules?.rules.some(
      (rule) => rule.kind === TriggerType.Effect && rule.trigger.filter === EffectFilter.Debuff,
    );

    const icon =
      debuff === true ? 'Skill/ICON_Common_Abnormal_Poison' : 'Skill/ICON_CL_Skill_047_Heal';

    const illustrated =
      node.kind === WidgetKind.Icon || node.binding?.source === GroupDataSource.Effects;

    return {
      ...node,
      appearance: {
        ...node.appearance,
        iconPath: node.appearance.iconPath ?? (illustrated ? icon : null),
      },
      children: node.children.map((child) => this.node(child)),
    };
  }
}
