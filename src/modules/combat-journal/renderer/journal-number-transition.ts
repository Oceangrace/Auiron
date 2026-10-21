export class JournalNumberTransition {
  private frame = 0;
  private current: number | null = null;
  private target: number | null = null;

  public constructor(private readonly publish: (value: number | null) => void) {}

  public update(value: number | null, durationMs: number): void {
    const duration = window.matchMedia('(prefers-reduced-motion: reduce)').matches ? 0 : durationMs;

    if (value === this.target && duration !== 0) {
      return;
    }

    this.target = value;
    this.dispose();

    if (value === null || this.current === null || duration === 0) {
      this.current = value;
      this.publish(value);

      return;
    }

    const from = this.current;

    const started = performance.now();

    const advance = (now: number): void => {
      const progress = Math.min(1, (now - started) / duration);

      this.current = from + (value - from) * (1 - (1 - progress) ** 3);
      this.publish(this.current);

      if (progress < 1 === true) {
        this.frame = requestAnimationFrame(advance);
      } else {
        this.frame = 0;
      }
    };

    this.frame = requestAnimationFrame(advance);
  }

  public dispose(): void {
    cancelAnimationFrame(this.frame);
    this.frame = 0;
  }
}
