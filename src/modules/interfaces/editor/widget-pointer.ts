import type { WidgetNode } from '../model/widget-node';
import type { PointerTarget } from './pointer-target';
import type { WidgetLayout } from '../model/widget-layout';
import type { PointerGesture } from './pointer-gesture';

import { AnchorMode } from '../model/anchor-mode';
import type { WidgetAnchors } from '../model/widget-anchors';

import { WidgetSnapping } from './widget-snapping';
import type { ShallowRef } from 'vue';
import type { SnapGuides } from './snap-guides';

export class WidgetPointer {
  private readonly snapping = new WidgetSnapping();
  private gesture: PointerGesture | null = null;

  public constructor(private readonly session: PointerTarget) {}

  public getGuides(): Readonly<ShallowRef<SnapGuides>> {
    return this.snapping.getGuides();
  }

  public begin(
    event: PointerEvent,
    widget: WidgetNode,
    scale: number,
    resizing: boolean,
    rendered: WidgetLayout,
    movable = true,
    contentScale = 1,
  ): void {
    if (event.button !== 0 || this.session.isReadOnly() === true) {
      return;
    }

    if (event.currentTarget instanceof HTMLElement === false) {
      return;
    }

    event.preventDefault();
    event.stopPropagation();
    this.session.beginGesture(widget.id, resizing === false, scale);

    if (movable === false) {
      this.session.endGesture(true);

      return;
    }

    event.currentTarget.setPointerCapture(event.pointerId);
    const bounds = this.bounds(event.currentTarget, resizing);

    this.snapping.begin(event.currentTarget, this.session.selectionIds());

    this.gesture = {
      pointerId: event.pointerId,
      widgetId: widget.id,
      startX: event.clientX,
      startY: event.clientY,
      original: widget.layout,
      anchors: widget.anchors,
      rendered,
      scale,
      resizing,
      contentScale,
      screenX: bounds.left,
      screenY: bounds.top,
    };
  }

  private bounds(element: HTMLElement, resizing: boolean): DOMRect {
    return (resizing ? (element.parentElement ?? element) : element).getBoundingClientRect();
  }

  public move(event: PointerEvent): void {
    const gesture = this.gesture;

    if (gesture?.pointerId !== event.pointerId) {
      return;
    }

    const deltaX = Math.round((event.clientX - gesture.startX) / gesture.scale);

    const deltaY = Math.round((event.clientY - gesture.startY) / gesture.scale);

    const layout = gesture.resizing
      ? this.resizedLayout(gesture, deltaX, deltaY, event.shiftKey)
      : this.draggedLayout(gesture, deltaX, deltaY, event.altKey);

    this.session.updateLayout(gesture.widgetId, layout, this.resizedAnchors(gesture, layout));
  }

  private draggedLayout(
    gesture: PointerGesture,
    deltaX: number,
    deltaY: number,
    unsnapped: boolean,
  ): WidgetLayout {
    const layout = this.movedLayout(gesture, deltaX, deltaY);

    if (unsnapped === true) {
      this.snapping.clear();

      return layout;
    }

    const snapped = this.snapping.snap(gesture, layout);

    return this.movedLayout(
      gesture,
      snapped.x - gesture.original.x,
      snapped.y - gesture.original.y,
    );
  }

  private resizedLayout(
    gesture: PointerGesture,
    deltaX: number,
    deltaY: number,
    proportional: boolean,
  ): WidgetLayout {
    let width = Math.max(8, Math.min(8192, gesture.rendered.width + deltaX / gesture.contentScale));

    let height = Math.max(
      8,
      Math.min(8192, gesture.rendered.height + deltaY / gesture.contentScale),
    );

    if (proportional === true) {
      const ratio =
        Math.abs(deltaX / gesture.rendered.width) >= Math.abs(deltaY / gesture.rendered.height)
          ? width / gesture.rendered.width
          : height / gesture.rendered.height;

      const factor = Math.max(
        8 / Math.min(gesture.rendered.width, gesture.rendered.height),
        Math.min(ratio, 8192 / Math.max(gesture.rendered.width, gesture.rendered.height)),
      );

      width = gesture.rendered.width * factor;
      height = gesture.rendered.height * factor;
    }

    return { ...gesture.original, width, height };
  }

  private resizedAnchors(gesture: PointerGesture, layout: WidgetLayout): WidgetAnchors {
    if (gesture.resizing === false) {
      return gesture.anchors;
    }

    return {
      ...gesture.anchors,
      widthPercent: null,
      heightPercent: null,
      right:
        gesture.anchors.horizontal === AnchorMode.Stretch
          ? gesture.anchors.right - (layout.width - gesture.rendered.width)
          : gesture.anchors.right,
      bottom:
        gesture.anchors.vertical === AnchorMode.Stretch
          ? gesture.anchors.bottom - (layout.height - gesture.rendered.height)
          : gesture.anchors.bottom,
    };
  }

  private movedLayout(gesture: PointerGesture, deltaX: number, deltaY: number): WidgetLayout {
    const x = Math.max(
      -gesture.screenX / gesture.scale,
      Math.min(
        deltaX,
        (window.innerWidth - gesture.screenX) / gesture.scale -
          gesture.rendered.width * gesture.contentScale,
      ),
    );

    const y = Math.max(
      -gesture.screenY / gesture.scale,
      Math.min(
        deltaY,
        (window.innerHeight - gesture.screenY) / gesture.scale -
          gesture.rendered.height * gesture.contentScale,
      ),
    );

    return { ...gesture.original, x: gesture.original.x + x, y: gesture.original.y + y };
  }

  public cancel(): void {
    this.snapping.clear();

    if (this.gesture !== null) {
      this.gesture = null;
      this.session.endGesture(true);
    }
  }

  public end(event: PointerEvent): void {
    if (this.gesture?.pointerId !== event.pointerId) {
      return;
    }

    const cancelled = event.type === 'pointercancel' || event.type === 'lostpointercapture';

    this.gesture = null;
    this.snapping.clear();
    this.session.endGesture(cancelled);
  }
}
