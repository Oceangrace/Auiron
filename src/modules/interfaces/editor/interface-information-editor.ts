import type { InterfaceDocument } from '../model/interface-document';
import type { InterfaceLocalization } from '../model/interface-localization';

export class InterfaceInformationEditor {
  private readonly fields = ['name', 'description'] as const;

  public update(
    document: InterfaceDocument,
    change: Partial<Pick<InterfaceDocument, 'name' | 'author' | 'version' | 'description'>>,
    locale: string,
  ): InterfaceDocument {
    const updated = { ...document, ...change };

    if (document.imported === true) {
      updated.author = document.author;
      updated.version = document.version;
    }

    const localization = document.localization;

    const language = this.language(localization, locale);

    if (localization === undefined || language === null) {
      return this.preserveTranslations(document, updated);
    }

    let texts = { ...localization.translations[language] };

    for (const field of this.fields) {
      const value = change[field];

      const source = document[field];

      if (value === undefined || source.trim() === '') {
        continue;
      }

      updated[field] = source;
      texts = { ...texts, [source]: value };
    }

    return {
      ...updated,
      localization: {
        ...localization,
        translations: { ...localization.translations, [language]: texts },
      },
    };
  }

  private language(localization: InterfaceLocalization | undefined, locale: string): string | null {
    let code = locale.toLowerCase().replaceAll('_', '-');

    while (code !== '') {
      if (code === localization?.defaultLocale) {
        return null;
      }

      if (Object.hasOwn(localization?.translations ?? {}, code) === true) {
        return code;
      }

      const end = code.lastIndexOf('-');

      code = end === -1 ? '' : code.slice(0, end);
    }

    return null;
  }

  private preserveTranslations(
    document: InterfaceDocument,
    updated: InterfaceDocument,
  ): InterfaceDocument {
    const localization = document.localization;

    if (localization === undefined) {
      return updated;
    }

    const translations = Object.fromEntries(
      Object.entries(localization.translations).map(([language, dictionary]) => [
        language,
        this.renamedTexts(document, updated, dictionary),
      ]),
    );

    return { ...updated, localization: { ...localization, translations } };
  }

  private renamedTexts(
    document: InterfaceDocument,
    updated: InterfaceDocument,
    dictionary: Readonly<Record<string, string>>,
  ): Readonly<Record<string, string>> {
    let texts = dictionary;

    for (const field of this.fields) {
      const source = document[field];

      const value = updated[field];

      if (source === value || value.trim() === '' || Object.hasOwn(dictionary, source) === false) {
        continue;
      }

      if (Object.hasOwn(texts, value) === false) {
        texts = { ...texts, [value]: dictionary[source] ?? '' };
      }
    }

    return texts;
  }
}
