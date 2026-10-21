import { ref } from 'vue';
import type { Ref } from 'vue';
import type { OverlayPosition } from '../../../core/preferences/overlay-position';
import type { LayersLayoutState } from './layers-layout-state';
import { LayersPlacement } from './layers-placement';

export class LayersLayout {
  public static readonly MIN_WIDTH = 240;
  public static readonly MAX_WIDTH = 480;
  private static readonly KEY = 'aurion.editor.layers.';
  private readonly state = ref<LayersLayoutState>({
    placement: LayersPlacement.Right,
    width: 300,
    floating: { x: 880, y: 80, width: 335, height: 720 },
  });

  public constructor(
    private readonly storage: Storage,
    private readonly report: (error: unknown) => void,
  ) {}

  public getState(): Readonly<Ref<LayersLayoutState>> {
    return this.state;
  }

  public load(): void {
    try {
      const placement = Object.values(LayersPlacement).find(
        (value) => value === this.storage.getItem(LayersLayout.KEY + 'placement'),
      );

      this.state.value = {
        placement: placement ?? LayersPlacement.Right,
        width: this.read('width', 300, LayersLayout.MIN_WIDTH, LayersLayout.MAX_WIDTH),
        floating: this.readFloating(),
      };
    } catch (error) {
      this.report(error);
    }
  }

  public place(placement: LayersPlacement): void {
    this.state.value = { ...this.state.value, placement };
    this.save();
  }

  public resizeDock(width: number): void {
    this.state.value = {
      ...this.state.value,
      width: Math.round(Math.max(LayersLayout.MIN_WIDTH, Math.min(LayersLayout.MAX_WIDTH, width))),
    };
  }

  public moveFloating(position: OverlayPosition): void {
    this.resizeFloating({ ...this.state.value.floating, ...position });
  }

  public resizeFloating(floating: LayersLayoutState['floating']): void {
    this.state.value = { ...this.state.value, floating };
    this.save();
  }

  public save(): void {
    const state = this.state.value;

    const values = {
      placement: state.placement,
      width: state.width,
      x: state.floating.x,
      y: state.floating.y,
      floatingWidth: state.floating.width,
      height: state.floating.height,
    };

    try {
      for (const [key, value] of Object.entries(values)) {
        this.storage.setItem(LayersLayout.KEY + key, String(value));
      }
    } catch (error) {
      this.report(error);
    }
  }

  private read(key: string, fallback: number, minimum: number, maximum: number): number {
    const value = Number(this.storage.getItem(LayersLayout.KEY + key) ?? fallback);

    return Math.max(minimum, Math.min(maximum, Number.isFinite(value) ? value : fallback));
  }

  private readFloating(): LayersLayoutState['floating'] {
    const width = this.read('floatingWidth', 335, 240, Math.max(240, window.innerWidth - 32));

    const height = this.read('height', 720, 260, Math.max(260, window.innerHeight - 60));

    return {
      x: this.read('x', 880, 0, Math.max(0, window.innerWidth - width - 16)),
      y: this.read('y', 80, 0, Math.max(0, window.innerHeight - height - 16)),
      width,
      height,
    };
  }
}
