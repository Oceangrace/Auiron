import { GroupGrow } from '../model/group-grow';

export class GroupGrowOffset {
  public resolve(grow: GroupGrow, horizontal: boolean, available: number, total: number): number {
    if (grow === GroupGrow.Center) {
      return (available - total) / 2;
    }

    const end =
      grow === GroupGrow.Backward ||
      (grow === GroupGrow.EndStart && horizontal) ||
      (grow === GroupGrow.StartEnd && !horizontal);

    return end ? available - total : 0;
  }
}
