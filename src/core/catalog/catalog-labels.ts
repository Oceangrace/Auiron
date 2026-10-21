import type { EffectKind } from './effect-kind';
import type { FieldBossDefinition } from './field-boss-definition';
export interface CatalogLabels {
  readonly fieldBosses?: readonly FieldBossDefinition[];
  readonly automaticSkills?: readonly number[];
  readonly hiddenEffects?: readonly number[];
  readonly zones?: Readonly<Record<number, string>>;
  readonly effectKinds?: Readonly<Record<number, EffectKind>>;
  readonly skillIcons?: Readonly<Record<number, string>>;
  readonly effectIcons?: Readonly<Record<number, string>>;
  readonly effects?: Readonly<Record<number, string>>;
  readonly skills: Readonly<Record<number, string>>;
  readonly npcs: Readonly<Record<number, string>>;
  readonly icons: Readonly<Record<string, string>>;
}
