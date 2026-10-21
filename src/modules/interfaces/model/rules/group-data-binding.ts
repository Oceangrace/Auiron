import { CombatDirection } from './combat-direction';
import { TableDimension } from './table-dimension';
import { CombatMetric } from './combat-metric';
import { GroupDataSource } from './group-data-source';
import { CombatScope } from './combat-scope';
import type { GroupEffectSettings } from './group-effect-settings';
import type { CombatTableView } from './combat-table-view';

export interface GroupDataBinding {
  readonly bossAlertMinutes?: readonly number[];
  readonly bossIds?: readonly number[];
  readonly effects?: GroupEffectSettings | null;
  readonly views: readonly CombatTableView[];
  readonly activeViewId: string | null;
  readonly showBreakdown: boolean;
  readonly direction: CombatDirection;
  readonly dimension: TableDimension;
  readonly actorId: string | null;
  readonly targetId: string | null;
  readonly includeSummons: boolean;
  readonly scope: CombatScope;
  readonly source: GroupDataSource;
  readonly metric: CombatMetric;
  readonly limit: number;
}

export const defaultGroupDataBinding: GroupDataBinding = {
  views: [],
  activeViewId: null,
  showBreakdown: false,
  direction: CombatDirection.Outgoing,
  dimension: TableDimension.Combatants,
  actorId: null,
  targetId: null,
  includeSummons: true,
  scope: CombatScope.Player,
  source: GroupDataSource.Game,
  metric: CombatMetric.Damage,
  limit: 10,
};
