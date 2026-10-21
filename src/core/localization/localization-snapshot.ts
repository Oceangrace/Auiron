import type { TranslationTree } from './translation-tree';
import type { LocaleOption } from './locale-option';

export interface LocalizationSnapshot {
  readonly defaultLocale: string;
  readonly fallbackLocale: string;
  readonly languages: readonly LocaleOption[];
  readonly messages: Readonly<Record<string, TranslationTree>>;
  readonly zoneValues: readonly string[];
  readonly zoneNames: Readonly<Record<string, Readonly<Record<string, string>>>>;
  readonly warnings: readonly string[];
}
