export interface SoundTone {
  readonly frequency: number;
  readonly startS: number;
  readonly lengthS: number;
  readonly endFrequency?: number;
  readonly wave?: OscillatorType;
  readonly gain?: number;
}
