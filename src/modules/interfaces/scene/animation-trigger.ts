import { AnimationChange } from '../model/animation-change';
export class AnimationTrigger {
  public changed(value: unknown, previous: unknown, change: AnimationChange): boolean {
    if (
      value === undefined ||
      previous === undefined ||
      value === null ||
      previous === null ||
      value === previous
    ) {
      return false;
    }

    if (change === AnimationChange.Any) {
      return true;
    }

    return this.numeric(value, previous, change);
  }

  private numeric(value: unknown, previous: unknown, change: AnimationChange): boolean {
    if (typeof value !== 'number' || typeof previous !== 'number') {
      return false;
    }

    return change === AnimationChange.Increase ? value > previous : value < previous;
  }
}
