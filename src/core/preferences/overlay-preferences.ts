import type { ScreenPreset } from './screen-preset';
import type { CharacterClass } from './character-class';
import type { OverlayAnchor } from './overlay-anchor';
import type { ToolbarLayout } from './toolbar-layout';
import type { OverlayPosition } from './overlay-position';
import type { PanelLayout } from './panel-layout';

export interface OverlayPreferences {
  readonly classColors: Readonly<Partial<Record<CharacterClass, string>>>;
  readonly locale: string;
  readonly screenPreset: ScreenPreset;
  readonly uiScale: number;
  readonly backgroundTransparency: number;
  readonly toolbarAnchor: OverlayAnchor;
  readonly toolbarLayout: ToolbarLayout;
  readonly toolbarPosition: OverlayPosition | null;
  readonly toolbarShowClock: boolean;
  readonly toolbarShowPlayTime: boolean;
  readonly toolbarAutoHide: boolean;
  readonly settingsPanel: PanelLayout | null;
}
