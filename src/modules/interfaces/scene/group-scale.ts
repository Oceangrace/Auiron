import type { WidgetNode } from '../model/widget-node';
import type { WidgetLayout } from '../model/widget-layout';
import { WidgetKind } from '../model/widget-kind';

export class GroupScale {
  public factor(widget: WidgetNode): number {
    if (widget.kind !== WidgetKind.Group && widget.kind !== WidgetKind.DynamicGroup) {
      return 1;
    }

    return widget.group.scale / 100;
  }

  public frame(widget: WidgetNode, layout: WidgetLayout = widget.layout): WidgetLayout {
    const factor = this.factor(widget);

    return { ...layout, width: layout.width * factor, height: layout.height * factor };
  }
}
