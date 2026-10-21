import type { SoundCue } from './sound-cue';
import type { GameClass } from './game-class';
import type { HealingPrediction } from './healing-prediction';
import type { WidgetAppearance } from './widget-appearance';
import type { WidgetStyle } from './widget-style';

export interface PreviewVisual {
  readonly className: GameClass | null;
  readonly simulating: boolean;
  readonly animationValue: string | number | boolean | null;
  readonly active: boolean | null;
  readonly visible: boolean | null;
  readonly remaining: number | null;
  readonly duration: number | null;
  readonly stacks: number | null;
  readonly knownProgress: boolean;
  readonly progressIdentity: string | null;
  readonly healingPrediction: HealingPrediction | null;
  readonly ticks: readonly number[];
  readonly animationActive: boolean;
  readonly sound: SoundCue;
  readonly speechText: string;
  readonly offsetX: number;
  readonly offsetY: number;
  readonly childVisuals: Readonly<Record<string, PreviewVisual>>;
  readonly instances: readonly { readonly key: string; readonly visual: PreviewVisual }[];
  readonly appearance: WidgetAppearance;
  readonly style: WidgetStyle;
}
