import type { InjectionKey } from 'vue';
import type { InterfacesModule } from './interfaces-module';

export const interfacesContextKey: InjectionKey<InterfacesModule> = Symbol('interfaces');
