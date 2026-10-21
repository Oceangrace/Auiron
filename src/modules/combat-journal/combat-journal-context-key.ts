import type { InjectionKey } from 'vue';
import type { CombatJournalModule } from './combat-journal-module';
export const combatJournalContextKey: InjectionKey<CombatJournalModule> = Symbol('combat-journal');
