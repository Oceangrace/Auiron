import type { WidgetLayout } from '../model/widget-layout';

import type { WidgetAnchors } from '../model/widget-anchors';

export interface PointerTarget {
  isReadOnly(): boolean;
  selectionIds(): readonly string[];
  beginGesture(id: string, moving?: boolean, scale?: number): void;
  updateLayout(id: string, layout: WidgetLayout, anchors: WidgetAnchors): void;
  endGesture(cancelled?: boolean): void;
}
