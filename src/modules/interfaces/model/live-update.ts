import type { LiveFrame } from './live-frame';
export interface LiveUpdate {
  readonly revision: number;
  readonly frame: LiveFrame | null;
}
