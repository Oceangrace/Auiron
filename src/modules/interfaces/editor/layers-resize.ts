import type { LayersLayout } from './layers-layout';
import { LayersPlacement } from './layers-placement';

export class LayersResize {
  private origin: { pointer: number; x: number; width: number; scale: number } | null = null;

  public constructor(private readonly layout: LayersLayout) {}

  public begin(event: PointerEvent): void {
    if (event.button !== 0 || event.currentTarget instanceof HTMLElement === false) {
      return;
    }

    const panel = event.currentTarget.parentElement;

    if (panel === null) {
      return;
    }

    event.preventDefault();
    event.currentTarget.setPointerCapture(event.pointerId);
    this.origin = {
      pointer: event.pointerId,
      x: event.clientX,
      width: panel.offsetWidth,
      scale: panel.getBoundingClientRect().width / panel.offsetWidth,
    };
  }

  public move(event: PointerEvent): void {
    const origin = this.origin;

    if (origin?.pointer !== event.pointerId) {
      return;
    }

    const direction = this.layout.getState().value.placement === LayersPlacement.Left ? 1 : -1;

    this.layout.resizeDock(origin.width + ((event.clientX - origin.x) / origin.scale) * direction);
  }

  public end(): void {
    if (this.origin === null) {
      return;
    }

    this.origin = null;
    this.layout.save();
  }

  public key(event: KeyboardEvent): void {
    if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') {
      return;
    }

    event.preventDefault();
    const direction = this.layout.getState().value.placement === LayersPlacement.Left ? 1 : -1;

    const delta = event.key === 'ArrowRight' ? 16 : -16;

    this.layout.resizeDock(this.layout.getState().value.width + delta * direction);
    this.layout.save();
  }
}
