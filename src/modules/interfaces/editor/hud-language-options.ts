import type { LocaleOption } from '../../../core/localization/locale-option';

export class HudLanguageOptions {
  private readonly codes = [
    'ru',
    'en',
    'de',
    'fr',
    'es',
    'it',
    'pt',
    'pt-br',
    'pl',
    'uk',
    'tr',
    'ko',
    'ja',
    'zh',
    'zh-tw',
  ];

  public choices(locale: string, installed: readonly LocaleOption[]): readonly LocaleOption[] {
    const names = new Intl.DisplayNames([locale], { type: 'language' });

    return [...new Set([...this.codes, ...installed.map((entry) => entry.value.toLowerCase())])]
      .map((value) => ({ value, label: names.of(value) ?? value }))
      .sort((left, right) => left.label.localeCompare(right.label, locale));
  }
}
