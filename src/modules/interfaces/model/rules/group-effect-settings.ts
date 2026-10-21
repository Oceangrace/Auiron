import { TriggerUnit } from './trigger-unit';
import { EffectFilter } from './effect-filter';

export interface GroupEffectSettings {
  readonly unit: TriggerUnit;
  readonly filter: EffectFilter;
  readonly iconSize: number;
}

export const defaultGroupEffectSettings: GroupEffectSettings = {
  unit: TriggerUnit.Player,
  filter: EffectFilter.Buff,
  iconSize: 32,
};
