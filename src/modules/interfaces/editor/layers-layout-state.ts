import type { LayersPlacement } from './layers-placement';

export interface LayersLayoutState {
  readonly placement: LayersPlacement;
  readonly width: number;
  readonly floating: { x: number; y: number; width: number; height: number };
}
