import type { InjectionKey } from 'vue';
import type { IconAssets } from './icon-assets';

export const iconAssetsContextKey: InjectionKey<IconAssets> = Symbol('icon-assets');
