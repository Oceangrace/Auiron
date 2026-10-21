import type { TranslationTree } from './translation-tree';

export class UserMessage {
  private readonly translations = new Map<string, ReadonlyMap<string, string>>();
  private readonly patterns = new Map<
    string,
    { canonical: string; pattern: RegExp; parameters: readonly string[] }
  >();

  public constructor(messages: Readonly<Record<string, TranslationTree>>) {
    for (const [locale, tree] of Object.entries(messages)) {
      const values = this.entries(tree);

      this.translations.set(locale, new Map(values));

      for (const [canonical, text] of values) {
        this.patterns.set(text, {
          canonical,
          pattern: this.pattern(text),
          parameters: this.parameters(text),
        });
      }
    }
  }

  public translate(text: string, locale: string): string {
    const value = text.replace(/^Error: /, '');

    for (const entry of this.patterns.values()) {
      const match = entry.pattern.exec(value);

      if (match !== null) {
        const parameters = new Map(
          entry.parameters.map((key, index) => [key, match[index + 1] ?? '']),
        );

        const translation = this.translations.get(locale)?.get(entry.canonical) ?? entry.canonical;

        let anonymous = 0;

        return translation.replace(/\{([^}]*)\}/g, (_parameter: string, key: string) => {
          const content = parameters.get(key === '' ? String(anonymous++) : key) ?? '';

          return key === 'error' ? this.translate(content, locale) : content;
        });
      }
    }

    return value;
  }

  private entries(tree: TranslationTree): readonly (readonly [string, string])[] {
    const errors = tree['errors'];

    return typeof errors === 'object'
      ? Object.entries(errors).flatMap(([key, value]) =>
          typeof value === 'string' ? [[key, value] as const] : [],
        )
      : [];
  }

  private pattern(source: string): RegExp {
    return new RegExp(
      '^' + source.replace(/[.*+?^$()|[\]\\]/g, '\\$&').replace(/\{[^}]*\}/g, '([\\s\\S]+?)') + '$',
    );
  }

  private parameters(source: string): readonly string[] {
    let anonymous = 0;

    return [...source.matchAll(/\{([^}]*)\}/g)].map((match) =>
      match[1] === '' ? String(anonymous++) : (match[1] ?? ''),
    );
  }
}
