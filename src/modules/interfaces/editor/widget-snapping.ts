import { shallowReadonly, shallowRef } from 'vue';
import type { ShallowRef } from 'vue';
import type { PointerGesture } from './pointer-gesture';
import type { WidgetLayout } from '../model/widget-layout';
import type { SnapGuides } from './snap-guides';
import { SnapAxis } from './snap-axis';

export class WidgetSnapping {
  private readonly horizontal = new SnapAxis();
  private readonly vertical = new SnapAxis();
  private readonly guides = shallowRef<SnapGuides>({ x: null, y: null });
  private xTargets: readonly number[] = [];
  private yTargets: readonly number[] = [];
  private bounds: DOMRect | null = null;

  public getGuides(): Readonly<ShallowRef<SnapGuides>> {
    return shallowReadonly(this.guides);
  }

  public begin(element: HTMLElement, selection: readonly string[]): void {
    this.clear();
    const surface = element.closest('.overlay-surface');

    const elements = Array.from(
      surface?.querySelectorAll<HTMLElement>('.scene-widget[data-widget-id]') ?? [],
    );

    const selected = elements.filter((entry) =>
      selection.includes(entry.dataset['widgetId'] ?? ''),
    );

    const moving = selected.filter(
      (entry) =>
        entry.hasAttribute('data-widget-movable') &&
        selected.some((parent) => parent !== entry && parent.contains(entry)) === false,
    );

    const targets = elements
      .filter((entry) => moving.some((current) => current.contains(entry)) === false)
      .map((entry) => entry.getBoundingClientRect());

    this.bounds = this.selectionBounds(moving);
    this.xTargets = [
      0,
      window.innerWidth / 2,
      window.innerWidth,
      ...targets.flatMap((rect) => [rect.left, rect.left + rect.width / 2, rect.right]),
    ];
    this.yTargets = [
      0,
      window.innerHeight / 2,
      window.innerHeight,
      ...targets.flatMap((rect) => [rect.top, rect.top + rect.height / 2, rect.bottom]),
    ];
  }

  public clear(): void {
    this.horizontal.reset();
    this.vertical.reset();
    this.guides.value = { x: null, y: null };
  }

  public snap(gesture: PointerGesture, layout: WidgetLayout): WidgetLayout {
    const bounds =
      this.bounds ??
      new DOMRect(
        gesture.screenX,
        gesture.screenY,
        gesture.rendered.width * gesture.scale * gesture.contentScale,
        gesture.rendered.height * gesture.scale * gesture.contentScale,
      );

    const left = bounds.left + (layout.x - gesture.original.x) * gesture.scale;

    const top = bounds.top + (layout.y - gesture.original.y) * gesture.scale;

    const width = bounds.width;

    const height = bounds.height;

    const x = this.horizontal.offset([left, left + width / 2, left + width], this.xTargets);

    const y = this.vertical.offset([top, top + height / 2, top + height], this.yTargets);

    this.guides.value = { x: this.horizontal.target(), y: this.vertical.target() };

    return { ...layout, x: layout.x + x / gesture.scale, y: layout.y + y / gesture.scale };
  }

  private selectionBounds(elements: readonly HTMLElement[]): DOMRect | null {
    if (elements.length === 0) {
      return null;
    }

    const rects = elements.map((element) => element.getBoundingClientRect());

    const left = Math.min(...rects.map((rect) => rect.left));

    const top = Math.min(...rects.map((rect) => rect.top));

    return new DOMRect(
      left,
      top,
      Math.max(...rects.map((rect) => rect.right)) - left,
      Math.max(...rects.map((rect) => rect.bottom)) - top,
    );
  }
}
