export interface TranslationTree {
  readonly [key: string]: string | TranslationTree;
}
