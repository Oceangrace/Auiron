export interface InterfaceLocalization {
  readonly defaultLocale: string;
  readonly translations: Readonly<Record<string, Readonly<Record<string, string>>>>;
}
