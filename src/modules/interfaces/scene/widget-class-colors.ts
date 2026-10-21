import { ClassPalette } from '../../../core/preferences/class-palette';
import type { CharacterClass } from '../../../core/preferences/character-class';
import type { PreviewVisual } from '../model/preview-visual';
import type { WidgetNode } from '../model/widget-node';
import { ClassColorSource } from '../model/class-color-source';
import { ClassColorTarget } from '../model/class-color-target';
import { WidgetKind } from '../model/widget-kind';

export class WidgetClassColors {
  private readonly palette = new ClassPalette();

  public apply(widget: WidgetNode, runtime: PreviewVisual | null): WidgetNode {
    const visual = { ...widget, ...(runtime ?? {}) };

    const name = runtime?.className ?? null;

    if (visual.style.classColorSource === ClassColorSource.None || name === null) {
      return visual;
    }

    return this.colored(visual, name);
  }

  private colored(visual: WidgetNode, name: CharacterClass): WidgetNode {
    const appearance = { ...visual.appearance };

    const style = { ...visual.style };

    const group = { ...visual.group };

    const color = (fallback: string): string => this.palette.color(name, fallback);

    switch (style.classColorTarget) {
      case ClassColorTarget.Fill:
        if (visual.kind === WidgetKind.Bar) {
          style.fillColor = color(style.fillColor);
          style.gradient = null;
        } else {
          appearance.background = color(appearance.background);
        }

        break;
      case ClassColorTarget.Background:
        appearance.background = color(appearance.background);
        break;
      case ClassColorTarget.Text:
        appearance.color = color(appearance.color);
        group.headerColor = color(group.headerColor);
        break;
      case ClassColorTarget.Border:
        appearance.borderColor = color(appearance.borderColor);
        break;
    }

    return { ...visual, appearance, style, group };
  }
}
