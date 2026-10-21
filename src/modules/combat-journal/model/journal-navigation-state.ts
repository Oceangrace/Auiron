import type { JournalAnalysisTab } from './journal-analysis-tab';
import type { JournalAnalysisSort } from './journal-analysis-sort';
import type { JournalMetric } from './journal-metric';
import type { JournalScope } from './journal-scope';

export interface JournalNavigationState {
  readonly fight: string | null;
  readonly tab: JournalAnalysisTab;
  readonly actor: number;
  readonly metric: JournalMetric;
  readonly scope: JournalScope;
  readonly mergeSummons: boolean;
  readonly target: string;
  readonly skill: string | null;
  readonly reference: string;
  readonly referenceActor: string;
  readonly bySkill: boolean;
  readonly raidCasts: boolean;
  readonly sort: JournalAnalysisSort;
  readonly search: string;
  readonly interval: readonly [number, number];
}
