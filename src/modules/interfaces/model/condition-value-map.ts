import type { ConditionProperty } from './condition-property';
import type { WidgetAnimation } from './widget-animation';
import type { FontWeight } from './font-weight';
import type { OutlineStyle } from './outline-style';
import type { GlowMode } from './glow-mode';
import type { SoundCue } from './sound-cue';
import type { ConditionAnimation } from './condition-animation';

export interface ConditionValueMap {
  readonly [ConditionProperty.FillColor]: string;
  readonly [ConditionProperty.Text]: string;
  readonly [ConditionProperty.Color]: string;
  readonly [ConditionProperty.Background]: string;
  readonly [ConditionProperty.BorderColor]: string;
  readonly [ConditionProperty.Opacity]: number;
  readonly [ConditionProperty.FontSize]: number;
  readonly [ConditionProperty.Glow]: boolean;
  readonly [ConditionProperty.Desaturate]: boolean;
  readonly [ConditionProperty.Animation]: WidgetAnimation;
  readonly [ConditionProperty.TextWeight]: FontWeight;
  readonly [ConditionProperty.TextOutline]: OutlineStyle;
  readonly [ConditionProperty.TextOutlineColor]: string;
  readonly [ConditionProperty.IconSwipe]: boolean;
  readonly [ConditionProperty.IconGlow]: GlowMode;
  readonly [ConditionProperty.IconGlowColor]: string;
  readonly [ConditionProperty.IconLabel]: string;
  readonly [ConditionProperty.BarGradient]: string | null;
  readonly [ConditionProperty.BarSpark]: boolean;
  readonly [ConditionProperty.BarInverse]: boolean;
  readonly [ConditionProperty.AnimationConfig]: ConditionAnimation;
  readonly [ConditionProperty.Sound]: SoundCue;
}
