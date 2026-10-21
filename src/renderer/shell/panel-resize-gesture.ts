import type { ResizeDirection } from './resize-direction';

export interface PanelResizeGesture {
  readonly pointerId: number;
  readonly direction: ResizeDirection;
  readonly startX: number;
  readonly startY: number;
  readonly bounds: DOMRectReadOnly;
}
