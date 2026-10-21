import { AnchorMode } from '../model/anchor-mode';
import type { WidgetAnchors } from '../model/widget-anchors';
import type { WidgetLayout } from '../model/widget-layout';

export class WidgetAnchorGeometry {
  public resolve(
    layout: WidgetLayout,
    anchors: WidgetAnchors,
    parent: WidgetLayout | null,
  ): WidgetLayout {
    if (parent === null) {
      return layout;
    }

    const width = this.size(
      anchors.horizontal,
      layout.width,
      parent.width,
      layout.x,
      anchors.right,
      anchors.widthPercent,
      8,
      8192,
    );

    const height = this.size(
      anchors.vertical,
      layout.height,
      parent.height,
      layout.y,
      anchors.bottom,
      anchors.heightPercent,
      8,
      8192,
    );

    return {
      ...layout,
      width,
      height,
      x: this.position(anchors.horizontal, layout.x, parent.width, width),
      y: this.position(anchors.vertical, layout.y, parent.height, height),
    };
  }

  private size(
    mode: AnchorMode,
    own: number,
    parent: number,
    start: number,
    end: number,
    percent: number | null,
    min: number,
    max: number,
  ): number {
    if (mode === AnchorMode.Stretch) {
      return Math.max(min, Math.min(max, parent - start - end));
    }

    return percent === null ? own : Math.max(min, Math.min(max, (parent * percent) / 100));
  }

  private position(mode: AnchorMode, offset: number, parent: number, own: number): number {
    if (mode === AnchorMode.Center) {
      return (parent - own) / 2 + offset;
    }

    return mode === AnchorMode.End ? parent - own + offset : offset;
  }
}
