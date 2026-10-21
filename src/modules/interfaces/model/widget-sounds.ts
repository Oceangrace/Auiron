import { SoundCue } from './sound-cue';

export interface WidgetSounds {
  readonly show: SoundCue;
  readonly hide: SoundCue;
  readonly volume: number;
  readonly showText: string;
  readonly hideText: string;
  readonly voice: string;
  readonly voiceRate: number;
}

export const defaultWidgetSounds: WidgetSounds = {
  show: SoundCue.None,
  hide: SoundCue.None,
  volume: 70,
  showText: '',
  hideText: '',
  voice: '',
  voiceRate: 100,
};
