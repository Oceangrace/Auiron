import type { CatalogSelection } from './catalog-selection';
import type { TriggerGroup } from './rules/trigger-group';
import type { GroupDataBinding } from './rules/group-data-binding';
import type { GroupHeaderButton } from './rules/group-header-button';
import type { LoadRules } from './load-rules';
import type { WidgetSounds } from './widget-sounds';
import type { WidgetStyle } from './widget-style';
import type { GroupSettings } from './group-settings';
import type { WidgetCondition } from './widget-condition';
import type { WidgetKind } from './widget-kind';
import type { WidgetLayout } from './widget-layout';
import type { WidgetAppearance } from './widget-appearance';
import type { TriggerDefinition } from './trigger-definition';

import type { WidgetAnchors } from './widget-anchors';

export interface WidgetNode {
  readonly ruleSelections: Readonly<Record<string, readonly CatalogSelection[]>>;
  readonly imageSelection: CatalogSelection | null;
  readonly rules: TriggerGroup | null;
  readonly binding: GroupDataBinding | null;
  readonly headerButtons: readonly GroupHeaderButton[];
  readonly load: LoadRules;
  readonly sounds: WidgetSounds;
  readonly entityId: string | null;
  readonly id: string;
  readonly name: string;
  readonly kind: WidgetKind;
  readonly enabled: boolean;
  readonly anchors: WidgetAnchors;
  readonly layout: WidgetLayout;
  readonly appearance: WidgetAppearance;
  readonly style: WidgetStyle;
  readonly group: GroupSettings;
  readonly conditions: readonly WidgetCondition[];
  readonly trigger: TriggerDefinition | null;
  readonly children: readonly WidgetNode[];
}
