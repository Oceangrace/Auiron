import { shallowReadonly, shallowRef } from 'vue';
import type { ShallowRef } from 'vue';

export class TimerClock {
  // 20 Hz bounds timer drift to 50 ms; cooldown swipes animate independently in CSS.
  private static readonly INTERVAL_MS = 50;
  private readonly now = shallowRef(performance.now());
  private users = 0;
  private timer: ReturnType<typeof setInterval> | null = null;
  private snapshotAt = performance.now();

  public current(): Readonly<ShallowRef<number>> {
    return shallowReadonly(this.now);
  }

  public snapshot(): number {
    return this.snapshotAt;
  }

  public mark(at: number): void {
    this.snapshotAt = at;
  }

  public acquire(): void {
    this.users += 1;

    if (this.timer === null) {
      this.now.value = performance.now();
      this.timer = setInterval(() => {
        this.now.value = performance.now();
      }, TimerClock.INTERVAL_MS);
    }
  }

  public release(): void {
    this.users = Math.max(0, this.users - 1);

    if (this.users === 0 && this.timer !== null) {
      clearInterval(this.timer);
      this.timer = null;
    }
  }
}

export const timerClock = new TimerClock();
