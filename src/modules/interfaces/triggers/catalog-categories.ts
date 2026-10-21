import type { UiSelectOption } from '../../../renderer/components/ui/ui-select-option';

export class CatalogCategories {
  public constructor(
    private readonly translate: (key: string) => string,
    private readonly locale: () => string,
  ) {}

  public options(available: readonly string[]): readonly UiSelectOption<string>[] {
    const options = available.map((value) => this.option(value));

    return [
      { value: '', label: this.translate('modules.interfaces.categories.all') },
      ...this.sorted(options.filter((option) => option.group === undefined)),
      ...this.sorted(options.filter((option) => option.group !== undefined)),
    ];
  }

  private option(value: string): UiSelectOption<string> {
    if (value.startsWith('class:') === true) {
      return {
        value,
        label: this.translate('modules.interfaces.classes.' + value.slice(6)),
        group: this.translate('modules.interfaces.self'),
      };
    }

    return { value, label: this.translate('modules.interfaces.categories.' + value) };
  }

  private sorted(options: UiSelectOption<string>[]): UiSelectOption<string>[] {
    return options.sort((left, right) => left.label.localeCompare(right.label, this.locale()));
  }
}
