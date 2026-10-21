export class SnapAxis {
  // Screen pixels: acquire nearby edges, then hold them through small pointer movements.
  private static readonly ACQUIRE_DISTANCE = 10;
  private static readonly RELEASE_DISTANCE = 16;
  private match: { index: number; target: number } | null = null;

  public reset(): void {
    this.match = null;
  }

  public target(): number | null {
    return this.match?.target ?? null;
  }

  public offset(points: readonly number[], targets: readonly number[]): number {
    const point = this.match === null ? undefined : points[this.match.index];

    if (this.match !== null && point !== undefined) {
      const delta = this.match.target - point;

      if (Math.abs(delta) <= SnapAxis.RELEASE_DISTANCE === true) {
        return delta;
      }
    }

    this.match = this.nearest(points, targets);

    return this.match === null ? 0 : this.match.target - (points[this.match.index] ?? 0);
  }

  private nearest(
    points: readonly number[],
    targets: readonly number[],
  ): { index: number; target: number } | null {
    const matches = targets.flatMap((target) =>
      points.map((point, index) => ({ index, target, distance: Math.abs(target - point) })),
    );

    return (
      matches
        .filter((match) => match.distance <= SnapAxis.ACQUIRE_DISTANCE)
        .sort((left, right) => left.distance - right.distance)[0] ?? null
    );
  }
}
