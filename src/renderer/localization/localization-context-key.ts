import type { InjectionKey } from 'vue';
import type { LocalizationSnapshot } from '../../core/localization/localization-snapshot';

export const localizationContextKey: InjectionKey<LocalizationSnapshot> = Symbol('localization');
