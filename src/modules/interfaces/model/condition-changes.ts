import type { ConditionChange } from './condition-change';
import type { ConditionAnimation } from './condition-animation';
import { defaultConditionAnimation } from './condition-animation';
import { ConditionProperty } from './condition-property';
import { WidgetAnimation } from './widget-animation';
import { GlowMode } from './glow-mode';
import { FontWeight } from './font-weight';
import { OutlineStyle } from './outline-style';
import { SoundCue } from './sound-cue';

export class ConditionChanges {
  private readonly defaults: {
    readonly [Property in ConditionProperty]: Extract<ConditionChange, { property: Property }>;
  } = {
    [ConditionProperty.FillColor]: { property: ConditionProperty.FillColor, value: '#d6b26e' },
    [ConditionProperty.Text]: { property: ConditionProperty.Text, value: '' },
    [ConditionProperty.Color]: { property: ConditionProperty.Color, value: '#ef6461' },
    [ConditionProperty.Background]: { property: ConditionProperty.Background, value: '#141414cc' },
    [ConditionProperty.BorderColor]: { property: ConditionProperty.BorderColor, value: '#d6b26e' },
    [ConditionProperty.Opacity]: { property: ConditionProperty.Opacity, value: 100 },
    [ConditionProperty.FontSize]: { property: ConditionProperty.FontSize, value: 20 },
    [ConditionProperty.Glow]: { property: ConditionProperty.Glow, value: true },
    [ConditionProperty.Desaturate]: { property: ConditionProperty.Desaturate, value: true },
    [ConditionProperty.Animation]: {
      property: ConditionProperty.Animation,
      value: WidgetAnimation.Pulse,
    },
    [ConditionProperty.TextWeight]: {
      property: ConditionProperty.TextWeight,
      value: FontWeight.Bold,
    },
    [ConditionProperty.TextOutline]: {
      property: ConditionProperty.TextOutline,
      value: OutlineStyle.Thin,
    },
    [ConditionProperty.TextOutlineColor]: {
      property: ConditionProperty.TextOutlineColor,
      value: '#000000',
    },
    [ConditionProperty.IconSwipe]: { property: ConditionProperty.IconSwipe, value: true },
    [ConditionProperty.IconGlow]: { property: ConditionProperty.IconGlow, value: GlowMode.Proc },
    [ConditionProperty.IconGlowColor]: {
      property: ConditionProperty.IconGlowColor,
      value: '#d6b26e',
    },
    [ConditionProperty.IconLabel]: { property: ConditionProperty.IconLabel, value: '' },
    [ConditionProperty.BarGradient]: { property: ConditionProperty.BarGradient, value: '#ef6461' },
    [ConditionProperty.BarSpark]: { property: ConditionProperty.BarSpark, value: true },
    [ConditionProperty.BarInverse]: { property: ConditionProperty.BarInverse, value: true },
    [ConditionProperty.AnimationConfig]: {
      property: ConditionProperty.AnimationConfig,
      value: defaultConditionAnimation,
    },
    [ConditionProperty.Sound]: {
      property: ConditionProperty.Sound,
      value: SoundCue.Ping,
      speechText: '',
    },
  };

  public create(property: ConditionProperty): ConditionChange {
    return structuredClone(this.defaults[property]);
  }

  public withValue(
    change: ConditionChange,
    value: string | number | boolean | null,
  ): ConditionChange {
    if (value === null) {
      return change.property === ConditionProperty.BarGradient ? { ...change, value } : change;
    }

    if (typeof value === 'string') {
      return this.withString(change, value);
    }

    if (typeof value === 'number') {
      return this.withNumber(change, value);
    }

    return this.withFlag(change, value);
  }

  public withAnimation(change: ConditionChange, value: ConditionAnimation): ConditionChange {
    return change.property === ConditionProperty.AnimationConfig ? { ...change, value } : change;
  }

  public withSpeech(change: ConditionChange, speechText: string): ConditionChange {
    return change.property === ConditionProperty.Sound ? { ...change, speechText } : change;
  }

  private withString(change: ConditionChange, value: string): ConditionChange {
    return (
      this.withColor(change, value) ??
      this.withTextStyle(change, value) ??
      this.withChoice(change, value) ??
      this.withLabel(change, value)
    );
  }

  private withNumber(change: ConditionChange, value: number): ConditionChange {
    if (
      change.property === ConditionProperty.Opacity ||
      change.property === ConditionProperty.FontSize
    ) {
      return { ...change, value };
    }

    return change;
  }

  private withFlag(change: ConditionChange, value: boolean): ConditionChange {
    if (
      change.property === ConditionProperty.Glow ||
      change.property === ConditionProperty.Desaturate ||
      change.property === ConditionProperty.IconSwipe ||
      change.property === ConditionProperty.BarSpark ||
      change.property === ConditionProperty.BarInverse
    ) {
      return { ...change, value };
    }

    return change;
  }

  private withColor(change: ConditionChange, value: string): ConditionChange | null {
    if (
      change.property === ConditionProperty.Color ||
      change.property === ConditionProperty.FillColor ||
      change.property === ConditionProperty.Background ||
      change.property === ConditionProperty.BorderColor ||
      change.property === ConditionProperty.TextOutlineColor ||
      change.property === ConditionProperty.IconGlowColor
    ) {
      return { ...change, value };
    }

    return null;
  }

  private withLabel(change: ConditionChange, value: string): ConditionChange {
    if (
      change.property === ConditionProperty.Text ||
      change.property === ConditionProperty.IconLabel ||
      change.property === ConditionProperty.BarGradient
    ) {
      return { ...change, value };
    }

    return change;
  }

  private withTextStyle(change: ConditionChange, value: string): ConditionChange | null {
    if (change.property === ConditionProperty.TextWeight) {
      const choice = this.choice(FontWeight, value);

      return choice === undefined ? change : { ...change, value: choice };
    }

    if (change.property === ConditionProperty.TextOutline) {
      const choice = this.choice(OutlineStyle, value);

      return choice === undefined ? change : { ...change, value: choice };
    }

    return null;
  }

  private withChoice(change: ConditionChange, value: string): ConditionChange | null {
    if (change.property === ConditionProperty.Animation) {
      const choice = this.choice(WidgetAnimation, value);

      return choice === undefined ? change : { ...change, value: choice };
    }

    if (change.property === ConditionProperty.IconGlow) {
      const choice = this.choice(GlowMode, value);

      return choice === undefined ? change : { ...change, value: choice };
    }

    if (change.property === ConditionProperty.Sound) {
      const choice = this.choice(SoundCue, value);

      return choice === undefined ? change : { ...change, value: choice };
    }

    return null;
  }

  private choice<Value extends string>(
    values: Readonly<Record<string, Value>>,
    selected: string,
  ): Value | undefined {
    return Object.values(values).find((value) => value === selected);
  }
}
