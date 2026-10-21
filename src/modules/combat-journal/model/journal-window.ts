import type { JournalMetric } from './journal-metric';
import type { JournalPeriod } from './journal-period';
import type { JournalScope } from './journal-scope';
export interface JournalWindow {
  readonly id: string;
  readonly name: string;
  readonly visible: boolean;
  readonly locked: boolean;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly metric: JournalMetric;
  readonly scope: JournalScope;
  readonly period: JournalPeriod;
  readonly limit: number;
  readonly mergeSummons: boolean;
}
