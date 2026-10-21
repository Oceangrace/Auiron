import type { JournalNavigationState } from '../model/journal-navigation-state';

export class JournalNavigation {
  private static readonly LIMIT = 100;

  private entries: JournalNavigationState[];
  private index = 0;

  public constructor(initial: JournalNavigationState) {
    this.entries = [initial];
  }

  public get canBack(): boolean {
    return this.index > 0;
  }

  public get canForward(): boolean {
    return this.index < this.entries.length - 1;
  }

  public visit(state: JournalNavigationState): void {
    if (JSON.stringify(state) === JSON.stringify(this.entries[this.index])) {
      return;
    }

    this.entries = [...this.entries.slice(0, this.index + 1), state].slice(
      -JournalNavigation.LIMIT,
    );
    this.index = this.entries.length - 1;
  }

  public back(): JournalNavigationState | null {
    if (this.canBack === false) {
      return null;
    }

    this.index--;

    return this.entries[this.index] ?? null;
  }

  public update(state: JournalNavigationState): void {
    this.entries[this.index] = state;
  }

  public forward(): JournalNavigationState | null {
    if (this.canForward === false) {
      return null;
    }

    this.index++;

    return this.entries[this.index] ?? null;
  }
}
