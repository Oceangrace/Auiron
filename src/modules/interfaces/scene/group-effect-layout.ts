import type { WidgetNode } from '../model/widget-node';
import type { WidgetLayout } from '../model/widget-layout';
import { GroupFlow } from '../model/group-flow';
import { GroupGrowOffset } from './group-grow-offset';

export class GroupEffectLayout {
  private readonly grow = new GroupGrowOffset();

  public resolve(node: WidgetNode, count: number, index: number): WidgetLayout {
    const size = node.binding?.effects?.iconSize ?? 32;

    const group = node.group;

    const [across, rows] = this.dimensions(node, count);

    const columns = across;

    const step = size + group.gap;

    const header = group.headerEnabled ? group.headerHeight : 0;

    const width = across * step - group.gap;

    const height = rows * step - group.gap;

    const x = group.flow === GroupFlow.Column ? 0 : index % columns;

    const y = group.flow === GroupFlow.Row ? 0 : Math.floor(index / across);

    return {
      x:
        group.padding +
        x * step +
        this.grow.resolve(group.grow, true, node.layout.width - group.padding * 2, width),
      y:
        group.padding +
        header +
        y * step +
        this.grow.resolve(
          group.grow,
          false,
          node.layout.height - group.padding * 2 - header,
          height,
        ),
      width: size,
      height: size,
    };
  }

  private dimensions(node: WidgetNode, count: number): readonly [number, number] {
    switch (node.group.flow) {
      case GroupFlow.Row:
        return [count, 1];
      case GroupFlow.Column:
        return [1, count];

      case GroupFlow.Grid:

      case GroupFlow.Timeline: {
        const columns = Math.max(1, Math.floor(node.group.columns));

        return [Math.min(columns, count), Math.ceil(count / columns)];
      }
    }
  }
}
