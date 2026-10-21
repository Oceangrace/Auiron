import { shallowRef, shallowReadonly } from 'vue';
import type { ShallowRef } from 'vue';
import type { DesktopCommands } from '../../core/desktop/desktop-commands';
import type { CatalogLabels } from '../../core/catalog/catalog-labels';
import type { JournalSegment } from './model/journal-segment';
import type { JournalPreferences } from './model/journal-preferences';
import type { JournalFrame } from './model/journal-frame';
import type { JournalWindow } from './model/journal-window';
import type { JournalAppearance } from './model/journal-appearance';
import type { JournalPanelLayout } from './model/journal-panel-layout';
import { JournalActions } from './runtime/journal-actions';
import { JournalHistory } from './runtime/journal-history';
import { JournalQuery } from './runtime/journal-query';
import { JournalFormatter } from './runtime/journal-formatter';
import { JournalPreview } from './runtime/journal-preview';

export class CombatJournalModule {
  public readonly history = new JournalHistory();
  public readonly formatter: JournalFormatter;
  public readonly query: JournalQuery;
  public readonly preview: JournalPreview;
  private readonly actions: JournalActions;
  private readonly preferences = shallowRef<JournalPreferences | null>(null);
  private readonly frame = shallowRef<JournalFrame | null>(null);
  private readonly error = shallowRef<string | null>(null);
  private readonly revision = shallowRef(0);
  private readonly settingsWindow = shallowRef<string | null>(null);
  private timer: ReturnType<typeof setTimeout> | null = null;
  private panelTimer: ReturnType<typeof setTimeout> | null = null;
  private disposed = false;
  private saves: Promise<void> = Promise.resolve();
  private archiveVersion = 0;
  private archiveWritable = true;
  private archiveFailures = 0;
  private archiveRetryAt = 0;

  public constructor(
    private readonly desktop: DesktopCommands,
    locale: () => string,
    labels: () => CatalogLabels,
  ) {
    this.actions = new JournalActions(labels);
    this.formatter = new JournalFormatter(locale, labels);
    this.preview = new JournalPreview();
    this.query = new JournalQuery(
      (id) => this.formatter.skill(id),
      (id, segment) => this.formatter.unit(id, segment),
    );
  }

  public getPreferences(): Readonly<ShallowRef<JournalPreferences | null>> {
    return shallowReadonly(this.preferences);
  }

  public getFrame(): Readonly<ShallowRef<JournalFrame | null>> {
    return shallowReadonly(this.frame);
  }

  public getError(): Readonly<ShallowRef<string | null>> {
    return shallowReadonly(this.error);
  }

  public getSettingsWindow(): Readonly<ShallowRef<string | null>> {
    return shallowReadonly(this.settingsWindow);
  }

  public openSettings(id: string): void {
    this.settingsWindow.value = id;
  }

  public closeSettings(): void {
    this.flushPanel();
    this.settingsWindow.value = null;
  }

  public savePanel(key: 'settingsPanel' | 'analysisPanel', layout: JournalPanelLayout): void {
    const preferences = this.preferences.value;

    if (preferences !== null) {
      this.preferences.value = { ...preferences, [key]: layout };

      if (this.panelTimer !== null) {
        clearTimeout(this.panelTimer);
      }

      this.panelTimer = setTimeout(() => {
        this.flushPanel();
      }, 200);
    }
  }

  private flushPanel(): void {
    if (this.panelTimer === null) {
      return;
    }

    clearTimeout(this.panelTimer);
    this.panelTimer = null;

    if (this.preferences.value !== null) {
      this.save(this.preferences.value);
    }
  }

  public updateAppearance(patch: Partial<JournalAppearance>): void {
    const preferences = this.preferences.value;

    if (preferences !== null) {
      this.save({ ...preferences, appearance: { ...preferences.appearance, ...patch } });
    }
  }

  public getRevision(): Readonly<ShallowRef<number>> {
    return shallowReadonly(this.revision);
  }

  public isVisible(): boolean {
    const preferences = this.preferences.value;

    return (
      preferences?.enabled === true && preferences.windows.some((window) => window.visible === true)
    );
  }

  public async start(): Promise<void> {
    try {
      this.preferences.value = await this.desktop.invoke<JournalPreferences>(
        'journal_preferences_read',
      );
    } catch (error) {
      this.error.value = String(error);
    }

    try {
      const reports = await this.desktop.invoke<readonly JournalSegment[]>('journal_archive_read');

      this.history.restore(
        reports.map((report) => ({
          ...report,
          actions: this.actions.classify(report.actions ?? []),
        })),
        this.preferences.value?.historyLimit ?? 50,
      );
    } catch (error) {
      this.archiveWritable = false;
      this.error.value = String(error);
    }

    await this.poll();
  }

  private async poll(): Promise<void> {
    if (this.disposed === true) {
      return;
    }

    try {
      const frame = await this.nextFrame();

      if (this.isDisposed() === true) {
        return;
      }

      this.history.update(frame, this.preferences.value?.historyLimit ?? 50);
      this.persistHistory();
      this.frame.value = frame;
      this.revision.value++;
    } catch (error) {
      this.error.value = String(error);
    }

    if (this.isDisposed() === false) {
      this.timer = setTimeout(() => {
        void this.poll();
      }, 500);
    }
  }

  private nextFrame(): Promise<JournalFrame> {
    return this.desktop.invoke<JournalFrame>('journal_frame', {
      observationId: this.frame.value?.observationId ?? null,
      afterSequence: this.frame.value?.eventSequence ?? null,
    });
  }

  private persistHistory(): void {
    const version = this.history.archiveVersion();

    if (
      version === this.archiveVersion ||
      this.archiveWritable === false ||
      performance.now() < this.archiveRetryAt === true
    ) {
      return;
    }

    this.archiveVersion = version;
    const reports = this.history.archived();

    this.saves = this.saves
      .then(async () => {
        await this.desktop.invoke('journal_archive_save', { reports });
        this.archiveFailures = 0;
      })
      .catch((error: unknown) => {
        this.error.value = String(error);
        this.retryArchive(version);
      });
  }

  private retryArchive(version: number): void {
    if (this.archiveVersion === version) {
      this.archiveVersion = -1;
    }

    this.archiveRetryAt =
      performance.now() + Math.min(30_000, 1000 * (2 ** this.archiveFailures - 1));
    this.archiveFailures++;
  }

  public toggle(): void {
    const preferences = this.preferences.value;

    if (preferences !== null) {
      const visible = preferences.windows.some((window) => window.visible);

      const windows =
        visible === true
          ? preferences.windows
          : preferences.windows.map((window) => ({ ...window, visible: true }));

      this.save({
        ...preferences,
        enabled: this.isVisible() === false,
        windows,
      });
    }
  }

  public clearHistory(): void {
    this.history.clearHistory();
    this.persistHistory();
    this.revision.value++;
  }

  public update(window: JournalWindow): void {
    const preferences = this.preferences.value;

    if (preferences !== null) {
      this.save({
        ...preferences,
        windows: preferences.windows.map((entry) => (entry.id === window.id ? window : entry)),
      });
    }
  }

  public duplicate(id: string): void {
    const preferences = this.preferences.value;

    const window = preferences?.windows.find((entry) => entry.id === id);

    if (preferences === null || window === undefined || preferences.windows.length >= 8 === true) {
      return;
    }

    this.save({
      ...preferences,
      windows: [
        ...preferences.windows,
        {
          ...window,
          id: crypto.randomUUID(),
          name: window.name + ' 2',
          x: window.x + 32,
          y: window.y + 32,
        },
      ],
    });
  }

  public remove(id: string): void {
    const preferences = this.preferences.value;

    if (preferences === null || preferences.windows.length === 1) {
      return;
    }

    const windows = preferences.windows.filter((entry) => entry.id !== id);

    if (this.settingsWindow.value === id) {
      this.settingsWindow.value = windows[0]?.id ?? null;
    }

    this.save({ ...preferences, windows });
  }

  public close(id: string): void {
    const preferences = this.preferences.value;

    const window = preferences?.windows.find((entry) => entry.id === id);

    if (preferences === null || window === undefined) {
      return;
    }

    if (preferences.windows.length > 1 === true) {
      this.remove(id);

      return;
    }

    this.update({ ...window, visible: false });
  }

  private save(preferences: JournalPreferences): void {
    this.preferences.value = preferences;
    this.saves = this.saves
      .then(async () => {
        await this.desktop.invoke('journal_preferences_save', { preferences });
        this.error.value = null;
      })
      .catch((error: unknown) => {
        this.error.value = String(error);
      });
  }

  public flush(): Promise<void> {
    this.flushPanel();
    this.archiveRetryAt = 0;
    this.persistHistory();

    return this.saves;
  }

  public async exportReport(text: string): Promise<void> {
    await this.desktop.invoke('journal_export_file', { text });
  }

  private isDisposed(): boolean {
    return this.disposed;
  }

  public dispose(): void {
    this.flushPanel();
    this.disposed = true;

    if (this.timer !== null) {
      clearTimeout(this.timer);
    }
  }
}
