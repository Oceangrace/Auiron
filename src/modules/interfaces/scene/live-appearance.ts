import { shallowReadonly, shallowRef } from 'vue';
import type { ShallowRef } from 'vue';
import type { DesktopCommands } from '../../../core/desktop/desktop-commands';
import type { LiveUpdate } from '../model/live-update';
import type { LiveFrame } from '../model/live-frame';
import { SourceStatus } from '../model/source-status';
import { timerClock } from './timer-clock';

export class LiveAppearance {
  private readonly frame = shallowRef<LiveFrame | null>(null);
  private readonly error = shallowRef<string | null>(null);
  private timer: ReturnType<typeof setTimeout> | null = null;
  private running = false;
  private revision: number | null = null;
  private interval = 100;

  public constructor(
    private readonly desktop: DesktopCommands,
    private readonly locale: () => string,
  ) {}

  public getFrame(): Readonly<ShallowRef<LiveFrame | null>> {
    return shallowReadonly(this.frame);
  }

  public getError(): Readonly<ShallowRef<string | null>> {
    return shallowReadonly(this.error);
  }

  public start(): void {
    if (this.running === true) {
      return;
    }

    this.running = true;
    void this.refresh();
  }

  private async refresh(): Promise<void> {
    this.timer = null;

    try {
      const update = await this.desktop.invoke<LiveUpdate>('interfaces_live', {
        locale: this.locale(),
        revision: this.revision,
      });

      this.accept(update);
    } catch (error) {
      if (this.running === true) {
        this.revision = null;
        this.frame.value = null;
        this.error.value = String(error);
      }
    } finally {
      if (this.running === true) {
        this.timer = setTimeout(() => {
          void this.refresh();
        }, this.interval);
      }
    }
  }

  private accept(update: LiveUpdate): void {
    if (this.running === false) {
      return;
    }

    this.revision = update.revision;

    if (update.frame !== null) {
      timerClock.mark(performance.now() - this.age(update.frame));
      this.frame.value = update.frame;
      this.error.value = update.frame.error;
    }
  }

  // A live snapshot carries Unix time, so its age on arrival is known; replay time is not
  // wall-clock time and gets no correction. Clamped against clock adjustments.
  private age(frame: LiveFrame): number {
    if (frame.status !== SourceStatus.Live || frame.atUs === null) {
      return 0;
    }

    return Math.max(0, Math.min(1000, Date.now() - frame.atUs / 1000));
  }

  public setForeground(active: boolean): void {
    const interval = active ? 100 : 500;

    if (interval === this.interval) {
      return;
    }

    this.interval = interval;

    if (this.running === true && this.timer !== null) {
      clearTimeout(this.timer);
      this.timer = null;
      void this.refresh();
    }
  }

  public dispose(): void {
    this.running = false;

    if (this.timer !== null) {
      clearTimeout(this.timer);
    }

    this.timer = null;
  }
}
