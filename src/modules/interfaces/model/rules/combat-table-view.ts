import type { CombatMetric } from './combat-metric';
import type { CombatDirection } from './combat-direction';
import type { TableDimension } from './table-dimension';
import type { CombatScope } from './combat-scope';

export interface CombatTableView {
  readonly id: string;
  readonly name: string;
  readonly metric: CombatMetric;
  readonly direction: CombatDirection;
  readonly dimension: TableDimension;
  readonly scope: CombatScope;
  readonly actorId: string | null;
  readonly targetId: string | null;
}
