export interface UiSelectOption<T extends string> {
  readonly value: T;
  readonly label: string;
  readonly group?: string;
  readonly parentGroup?: string;
}
