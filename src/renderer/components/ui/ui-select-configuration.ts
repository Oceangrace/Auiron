import type { UiSelectOption } from './ui-select-option';

export interface UiSelectConfiguration<T extends string> {
  readonly modelValue: T;
  readonly options: readonly UiSelectOption<T>[];
  readonly allOptions?: readonly UiSelectOption<T>[];
  readonly disabled: boolean;
  readonly popupMinWidth?: number;
  readonly compact?: boolean;
  readonly remoteSearch?: boolean;
}
