export class TrayNoticeTimer {
  public static readonly durationMs = 10_000;
  private remaining = TrayNoticeTimer.durationMs;
  private started = 0;
  private timer: ReturnType<typeof setTimeout> | null = null;

  public constructor(private readonly finish: () => void) {}

  public resume(): void {
    if (this.timer !== null) {
      return;
    }

    this.started = performance.now();
    this.timer = setTimeout(() => {
      this.timer = null;
      this.remaining = 0;
      this.finish();
    }, this.remaining);
  }

  public pause(): void {
    if (this.timer === null) {
      return;
    }

    clearTimeout(this.timer);
    this.timer = null;
    this.remaining = Math.max(0, this.remaining - (performance.now() - this.started));
  }

  public dispose(): void {
    this.pause();
  }
}
