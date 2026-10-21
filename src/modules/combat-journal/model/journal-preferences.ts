import type { JournalAppearance } from './journal-appearance';
import type { JournalWindow } from './journal-window';
import type { JournalPanelLayout } from './journal-panel-layout';

export interface JournalPreferences {
  readonly enabled: boolean;
  readonly historyLimit: number;
  readonly appearance: JournalAppearance;
  readonly settingsPanel?: JournalPanelLayout | null;
  readonly analysisPanel?: JournalPanelLayout | null;
  readonly windows: readonly JournalWindow[];
}
