import { GameClass } from '../model/game-class';
import type { UiSelectOption } from '../../../renderer/components/ui/ui-select-option';

export class LoadOptions {
  public constructor(
    private readonly translate: (key: string) => string,
    private readonly locale: () => string,
    private readonly values: readonly string[],
    private readonly names: Readonly<Record<string, Readonly<Record<string, string>>>>,
  ) {}

  public classes(current: string): readonly UiSelectOption<string>[] {
    return this.preserve(current, [
      this.any(),
      ...Object.values(GameClass)
        .map((value) => ({
          value,
          label: this.translate('modules.interfaces.classes.' + value),
        }))
        .sort((left, right) => left.label.localeCompare(right.label, this.locale())),
    ]);
  }

  public zones(current: string): readonly UiSelectOption<string>[] {
    return this.preserve(current, [
      this.any(),
      ...this.values
        .map((value) => ({
          value,
          label: this.names[this.locale()]?.[value] ?? value,
        }))
        .sort((left, right) => left.label.localeCompare(right.label, this.locale())),
    ]);
  }

  private any(): UiSelectOption<string> {
    return { value: '', label: this.translate('modules.interfaces.advanced.options.any') };
  }

  private preserve(
    current: string,
    options: readonly UiSelectOption<string>[],
  ): readonly UiSelectOption<string>[] {
    if (options.some((option) => option.value === current) === true) {
      return options;
    }

    return [
      ...options,
      {
        value: current,
        label: current,
        group: this.translate('modules.interfaces.advanced.savedValue'),
      },
    ];
  }
}
