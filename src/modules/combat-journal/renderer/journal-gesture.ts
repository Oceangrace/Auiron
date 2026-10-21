import type { JournalWindow } from '../model/journal-window';
import { JournalGestureMode } from '../model/journal-gesture-mode';

export class JournalGesture {
  private active: {
    pointerId: number;
    x: number;
    y: number;
    origin: JournalWindow;
    mode: JournalGestureMode;
    element: HTMLElement;
  } | null = null;

  public constructor(
    private readonly read: () => JournalWindow,
    private readonly preview: (value: JournalWindow | null) => void,
    private readonly commit: (value: JournalWindow) => void,
  ) {}

  public begin(event: PointerEvent, mode: JournalGestureMode): void {
    if (
      event.button !== 0 ||
      this.read().locked === true ||
      event.currentTarget instanceof HTMLElement === false
    ) {
      return;
    }

    if (
      mode === JournalGestureMode.Move &&
      event.target instanceof Element === true &&
      event.target.closest('button, input') !== null
    ) {
      return;
    }

    event.preventDefault();
    this.active = {
      pointerId: event.pointerId,
      x: event.clientX,
      y: event.clientY,
      origin: this.visibleOrigin(event.currentTarget),
      mode,
      element: event.currentTarget,
    };
    event.currentTarget.setPointerCapture(event.pointerId);
  }

  private visibleOrigin(element: HTMLElement): JournalWindow {
    const bounds = element.closest('.journal-window')?.getBoundingClientRect();

    return bounds === undefined
      ? this.read()
      : {
          ...this.read(),
          x: bounds.x,
          y: bounds.y,
          width: Math.max(220, bounds.width),
          height: Math.max(100, bounds.height),
        };
  }

  public move(event: PointerEvent): void {
    const active = this.active;

    if (active?.pointerId !== event.pointerId) {
      return;
    }

    const dx = event.clientX - active.x;

    const dy = event.clientY - active.y;

    const origin = active.origin;

    if (active.mode === JournalGestureMode.Move) {
      this.preview({
        ...origin,
        x: Math.max(0, Math.min(window.innerWidth - Math.min(80, origin.width), origin.x + dx)),
        y: Math.max(0, Math.min(window.innerHeight - 25, origin.y + dy)),
      });

      return;
    }

    this.preview({
      ...origin,
      width:
        active.mode === JournalGestureMode.Size
          ? Math.max(220, Math.min(1920, window.innerWidth - origin.x, origin.width + dx))
          : origin.width,
      height: Math.max(100, Math.min(1440, window.innerHeight - origin.y, origin.height + dy)),
    });
  }

  public finish(value: JournalWindow): void {
    if (this.active === null) {
      return;
    }

    this.cancel();
    this.commit(value);
  }

  public cancel(): void {
    const active = this.active;

    this.active = null;
    this.preview(null);

    if (active !== null && active.element.hasPointerCapture(active.pointerId) === true) {
      active.element.releasePointerCapture(active.pointerId);
    }
  }
}
