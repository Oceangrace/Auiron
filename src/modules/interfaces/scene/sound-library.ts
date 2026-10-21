import { SoundCue } from '../model/sound-cue';
import { SoundGroup } from '../model/sound-group';
import type { SoundTone } from './sound-tone';
import type { UiSelectOption } from '../../../renderer/components/ui/ui-select-option';
export class SoundLibrary {
  public readonly tones: Readonly<Record<SoundCue, readonly SoundTone[]>> = {
    [SoundCue.None]: [],
    [SoundCue.Speech]: [],
    [SoundCue.Ping]: this.notes([1320], 0, 0.18),
    [SoundCue.Chime]: this.notes([880, 1320], 0.12, 0.24),
    [SoundCue.Alert]: this.notes([990, 990, 990], 0.16, 0.1),
    [SoundCue.Low]: this.notes([330], 0, 0.35),
    [SoundCue.Bell]: [
      ...this.notes([1046], 0, 0.8),
      { frequency: 2092, startS: 0, lengthS: 0.35, gain: 0.25 },
    ],
    [SoundCue.DoubleBell]: this.notes([1174, 1174], 0.24, 0.4),
    [SoundCue.Crystal]: this.notes([1568, 2093, 2637], 0.09, 0.3),
    [SoundCue.Harp]: this.notes([523, 659, 784, 1046], 0.07, 0.45, 'triangle'),
    [SoundCue.Marimba]: this.notes([523, 784, 659], 0.13, 0.13, 'triangle'),
    [SoundCue.Glass]: this.notes([1760, 2349], 0.06, 0.4),
    [SoundCue.Drop]: [{ frequency: 1400, endFrequency: 440, startS: 0, lengthS: 0.12 }],
    [SoundCue.Pop]: [
      { frequency: 420, endFrequency: 90, startS: 0, lengthS: 0.065, wave: 'triangle' },
    ],
    [SoundCue.Click]: this.notes([1800], 0, 0.025, 'triangle'),
    [SoundCue.Knock]: this.notes([160, 130], 0.11, 0.065, 'triangle'),
    [SoundCue.Rise]: [
      { frequency: 440, endFrequency: 1760, startS: 0, lengthS: 0.4, wave: 'triangle' },
    ],
    [SoundCue.Fall]: [
      { frequency: 1320, endFrequency: 220, startS: 0, lengthS: 0.4, wave: 'triangle' },
    ],
    [SoundCue.Success]: this.notes([523, 659, 784, 1046], 0.11, 0.2),
    [SoundCue.Failure]: this.notes([440, 370, 294], 0.15, 0.24, 'triangle'),
    [SoundCue.Ready]: this.notes([784, 1046], 0.14, 0.28, 'triangle'),
    [SoundCue.Countdown]: this.notes([660, 660, 1320], 0.35, 0.12),
    [SoundCue.Warning]: this.notes([880, 660, 880, 660], 0.15, 0.12, 'square'),
    [SoundCue.Siren]: [
      { frequency: 500, endFrequency: 1300, startS: 0, lengthS: 0.45, wave: 'triangle' },
      { frequency: 1300, endFrequency: 500, startS: 0.45, lengthS: 0.45, wave: 'triangle' },
    ],
    [SoundCue.Radar]: this.notes([1800, 1800, 1800], 0.22, 0.045),
    [SoundCue.Sonar]: this.notes([740], 0, 0.95),
    [SoundCue.Horn]: [
      ...this.notes([220], 0, 0.5, 'sawtooth'),
      { frequency: 277, startS: 0, lengthS: 0.5, gain: 0.45, wave: 'sawtooth' },
    ],
    [SoundCue.Buzz]: this.notes([140, 140], 0.24, 0.16, 'sawtooth'),
    [SoundCue.Heartbeat]: this.notes([85, 65, 85, 65], 0.2, 0.12, 'triangle'),
    [SoundCue.Tick]: this.notes([2200, 1800], 0.1, 0.025),
  };

  public options(translate: (key: string) => string): UiSelectOption<SoundCue>[] {
    return Object.values(SoundCue).map((value) => ({
      value,
      label: translate('modules.interfaces.audio.cues.' + value),
      group:
        value === SoundCue.None
          ? ''
          : translate('modules.interfaces.audio.groups.' + this.group(value)),
    }));
  }

  private notes(
    frequencies: readonly number[],
    spacing: number,
    lengthS: number,
    wave: OscillatorType = 'sine',
  ): SoundTone[] {
    return frequencies.map((frequency, index) => ({
      frequency,
      startS: index * spacing,
      lengthS,
      wave,
    }));
  }

  private group(cue: SoundCue): SoundGroup {
    if (cue === SoundCue.Speech) {
      return SoundGroup.Voice;
    }

    if (
      [
        SoundCue.Alert,
        SoundCue.Low,
        SoundCue.Warning,
        SoundCue.Siren,
        SoundCue.Horn,
        SoundCue.Buzz,
        SoundCue.Heartbeat,
      ].includes(cue) === true
    ) {
      return SoundGroup.Alarms;
    }

    if (
      [
        SoundCue.Bell,
        SoundCue.DoubleBell,
        SoundCue.Crystal,
        SoundCue.Harp,
        SoundCue.Marimba,
        SoundCue.Glass,
        SoundCue.Drop,
        SoundCue.Pop,
      ].includes(cue) === true
    ) {
      return SoundGroup.Soft;
    }

    return [
      SoundCue.Rise,
      SoundCue.Fall,
      SoundCue.Success,
      SoundCue.Failure,
      SoundCue.Ready,
      SoundCue.Countdown,
    ].includes(cue)
      ? SoundGroup.Game
      : SoundGroup.Signals;
  }
}
