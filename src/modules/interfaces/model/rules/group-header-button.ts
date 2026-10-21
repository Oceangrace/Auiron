import type { GroupButtonDisplay } from './group-button-display';
import type { GroupButtonPlacement } from './group-button-placement';
import type { GroupHeaderAction } from './group-header-action';

export interface GroupHeaderButton {
  readonly id: string;
  readonly label: string;
  readonly display: GroupButtonDisplay;
  readonly placement: GroupButtonPlacement;
  readonly action: GroupHeaderAction;
  readonly icon: string;
}
