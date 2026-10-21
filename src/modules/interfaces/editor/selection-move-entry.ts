import type { WidgetNode } from '../model/widget-node';
import type { WidgetLayout } from '../model/widget-layout';

export interface SelectionMoveEntry {
  readonly node: WidgetNode;
  readonly screen: WidgetLayout;
  readonly scaleX: number;
  readonly scaleY: number;
}
