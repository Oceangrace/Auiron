export interface PreviewContext {
  readonly simulating?: boolean;
  readonly fields?: Readonly<Record<string, string | number | boolean | null>>;
  readonly widgets?: Readonly<
    Record<string, Readonly<Record<string, string | number | boolean | null>>>
  >;
  readonly rows?: readonly Readonly<Record<string, string | number | boolean | null>>[];
  readonly instances?: Readonly<
    Record<string, readonly Readonly<Record<string, string | number | boolean | null>>[]>
  >;
  readonly active: boolean | null;
  readonly remaining: number | null;
  readonly stacks: number | null;
  readonly progress: number | null;
}
