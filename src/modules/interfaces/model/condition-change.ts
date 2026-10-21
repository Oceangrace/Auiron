import type { ConditionProperty } from './condition-property';
import type { ConditionValueMap } from './condition-value-map';

export type ConditionChange = {
  [Property in ConditionProperty]: {
    readonly property: Property;
    readonly value: ConditionValueMap[Property];
  } & (Property extends ConditionProperty.Sound ? { readonly speechText?: string } : object);
}[ConditionProperty];
