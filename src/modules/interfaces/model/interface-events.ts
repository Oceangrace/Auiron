import type { InterfaceSnapshot } from './interface-snapshot';

export interface InterfaceEvents {
  readonly 'icons:changed': readonly string[];
  readonly 'app:quit-requested': null;
  readonly 'interfaces:changed': InterfaceSnapshot;
  readonly 'interfaces:open-requested': null;
  readonly 'overlay:game-panel': boolean;
  readonly 'overlay:foreground': boolean;
  readonly 'overlay:attached': boolean;
}
