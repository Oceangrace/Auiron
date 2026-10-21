import type { InterfaceDisplay } from '../model/interface-display';
import type { Viewport } from '../model/viewport';
import type { WidgetLayout } from '../model/widget-layout';

export class HudScale {
  public constructor(
    private readonly display: InterfaceDisplay | undefined,
    private readonly viewport: Viewport | null,
  ) {}

  public factor(): number {
    return this.ratio().y * ((this.display?.scale ?? 100) / 100);
  }

  public transform(root: WidgetLayout): string | undefined {
    const ratio = this.ratio();

    const factor = this.factor();

    if (factor === 1 && ratio.x === 1 && ratio.y === 1) {
      return undefined;
    }

    const x = (root.x + root.width / 2) * (ratio.x - factor);

    const y = (root.y + root.height / 2) * (ratio.y - factor);

    return `translate(${String(x)}px, ${String(y)}px) scale(${String(factor)})`;
  }

  public screen(root: WidgetLayout, layout: WidgetLayout): WidgetLayout {
    const ratio = this.ratio();

    const factor = this.factor();

    return {
      x: layout.x * factor + (root.x + root.width / 2) * (ratio.x - factor),
      y: layout.y * factor + (root.y + root.height / 2) * (ratio.y - factor),
      width: layout.width * factor,
      height: layout.height * factor,
    };
  }

  private ratio(): { readonly x: number; readonly y: number } {
    const display = this.display;

    if (display?.autoScale !== true || this.viewport === null) {
      return { x: 1, y: 1 };
    }

    const { baseWidth, baseHeight } = display;

    if (baseWidth === null || baseHeight === null) {
      return { x: 1, y: 1 };
    }

    return { x: this.viewport.width / baseWidth, y: this.viewport.height / baseHeight };
  }
}
