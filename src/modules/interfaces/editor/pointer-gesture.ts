import type { WidgetLayout } from '../model/widget-layout';

import type { WidgetAnchors } from '../model/widget-anchors';

export interface PointerGesture {
  readonly pointerId: number;
  readonly widgetId: string;
  readonly startX: number;
  readonly startY: number;
  readonly anchors: WidgetAnchors;
  readonly rendered: WidgetLayout;
  readonly original: WidgetLayout;
  readonly scale: number;
  readonly contentScale: number;
  readonly screenX: number;
  readonly screenY: number;
  readonly resizing: boolean;
}
