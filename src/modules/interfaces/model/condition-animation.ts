import { WidgetAnimation } from './widget-animation';
import { AnimationWhen } from './animation-when';
import { AnimationChange } from './animation-change';

export interface ConditionAnimation {
  readonly preset: WidgetAnimation;
  readonly when: AnimationWhen;
  readonly color: string;
  readonly periodMs: number;
  readonly intensity: number;
  readonly thresholdSeconds: number;
  readonly durationMs: number;
  readonly path: string;
  readonly change: AnimationChange;
}

export const defaultConditionAnimation: ConditionAnimation = {
  preset: WidgetAnimation.Pulse,
  when: AnimationWhen.Always,
  color: '#d6b26e',
  periodMs: 1000,
  intensity: 75,
  thresholdSeconds: 5,
  durationMs: 2000,
  path: 'combat.damage',
  change: AnimationChange.Any,
};
