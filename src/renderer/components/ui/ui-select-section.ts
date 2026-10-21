import type { UiSelectOption } from './ui-select-option';

export interface UiSelectSection<T extends string> {
  readonly key: string;
  readonly label: string;
  readonly expanded: boolean;
  readonly total: number;
  readonly options: readonly UiSelectOption<T>[];
  readonly children: readonly UiSelectSection<T>[];
}
