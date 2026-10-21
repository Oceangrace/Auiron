import type { OverlayPosition } from '../../core/preferences/overlay-position';
import type { PanelResizeGesture } from './panel-resize-gesture';
import { ResizeDirection } from './resize-direction';

export class PanelResizeController {
  private gesture: PanelResizeGesture | null = null;
  private enabled = true;
  private readonly onPointerDown = (event: PointerEvent): void => {
    if (this.enabled === false || event.button !== 0 || event.target instanceof Element === false) {
      return;
    }

    const handle = event.target.closest<HTMLElement>('[data-overlay-resize]');

    const direction = Object.values(ResizeDirection).find(
      (value) => value === handle?.dataset['overlayResize'],
    );

    if (direction === undefined) {
      return;
    }

    event.preventDefault();
    this.gesture = {
      pointerId: event.pointerId,
      direction,
      startX: event.clientX,
      startY: event.clientY,
      bounds: this.element.getBoundingClientRect(),
    };
    this.element.setPointerCapture(event.pointerId);
    this.element.dataset['resizing'] = direction;
  };

  private readonly onPointerMove = (event: PointerEvent): void => {
    const gesture = this.gesture;

    if (gesture?.pointerId !== event.pointerId) {
      return;
    }

    this.resize(gesture, event.clientX - gesture.startX, event.clientY - gesture.startY);
  };

  private readonly onPointerEnd = (): void => {
    const gesture = this.gesture;

    this.gesture = null;
    delete this.element.dataset['resizing'];

    if (gesture !== null && this.element.hasPointerCapture(gesture.pointerId) === true) {
      this.element.releasePointerCapture(gesture.pointerId);
    }
  };

  public constructor(
    private readonly element: HTMLElement,
    private readonly onResize: (
      position: OverlayPosition,
      size: { width: number; height: number },
    ) => void,
  ) {}

  public start(): void {
    this.element.addEventListener('pointerdown', this.onPointerDown);
    this.element.addEventListener('pointermove', this.onPointerMove);
    this.element.addEventListener('pointerup', this.onPointerEnd);
    this.element.addEventListener('pointercancel', this.onPointerEnd);
    this.element.addEventListener('lostpointercapture', this.onPointerEnd);
    window.addEventListener('blur', this.onPointerEnd);
  }

  public setEnabled(enabled: boolean): void {
    this.enabled = enabled;

    if (enabled === false) {
      this.onPointerEnd();
    }
  }

  public dispose(): void {
    this.onPointerEnd();
    this.element.removeEventListener('pointerdown', this.onPointerDown);
    this.element.removeEventListener('pointermove', this.onPointerMove);
    this.element.removeEventListener('pointerup', this.onPointerEnd);
    this.element.removeEventListener('pointercancel', this.onPointerEnd);
    this.element.removeEventListener('lostpointercapture', this.onPointerEnd);
    window.removeEventListener('blur', this.onPointerEnd);
  }

  private resize(gesture: PanelResizeGesture, deltaX: number, deltaY: number): void {
    const style = getComputedStyle(this.element);

    const bounds = gesture.bounds;

    const horizontal = this.resizeAxis(
      bounds.left,
      bounds.right,
      deltaX,
      Number.parseFloat(style.minWidth),
      window.innerWidth,
      gesture.direction.includes('w'),
      gesture.direction.includes('e'),
    );

    const vertical = this.resizeAxis(
      bounds.top,
      bounds.bottom,
      deltaY,
      Number.parseFloat(style.minHeight),
      window.innerHeight,
      gesture.direction.includes('n'),
      gesture.direction.includes('s'),
    );

    this.onResize(
      { x: horizontal[0], y: vertical[0] },
      { width: horizontal[1], height: vertical[1] },
    );
  }

  private resizeAxis(
    start: number,
    end: number,
    delta: number,
    minimum: number,
    viewport: number,
    leading: boolean,
    trailing: boolean,
  ): readonly [number, number] {
    const margin = 16;

    const minSize = Math.min(minimum, viewport - margin * 2);

    if (leading === true) {
      const next = Math.max(margin, Math.min(start + delta, end - minSize));

      return [next, end - next];
    }

    if (trailing === true) {
      const next = Math.min(viewport - margin, Math.max(end + delta, start + minSize));

      return [start, next - start];
    }

    return [start, end - start];
  }
}
