import type { SoundCue } from '../model/sound-cue';
export interface SoundPlayback {
  readonly cue: SoundCue;
  readonly volume: number;
  readonly text?: string;
  readonly voice?: string;
  readonly rate?: number;
  readonly owner: string;
}
