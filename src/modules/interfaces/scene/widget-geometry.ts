import { GroupScale } from './group-scale';
import type { WidgetNode } from '../model/widget-node';
import type { CSSProperties } from 'vue';
import type { WidgetLayout } from '../model/widget-layout';
import { WidgetKind } from '../model/widget-kind';
import { FontFamily } from '../model/font-family';
import { TextAlignment } from '../model/text-alignment';
import { TextAnchor } from '../model/text-anchor';
import { WidgetAnimation } from '../model/widget-animation';
export class WidgetGeometry {
  public style(widget: WidgetNode, layout: WidgetLayout): CSSProperties {
    const group = widget.kind === WidgetKind.Group || widget.kind === WidgetKind.DynamicGroup;

    layout = new GroupScale().frame(widget, layout);

    const style = widget.style;

    const families = {
      [FontFamily.Lora]: "'Lora Variable', Georgia, serif",
      [FontFamily.Sans]: 'system-ui, sans-serif',
      [FontFamily.Mono]: 'monospace',
      [FontFamily.Inter]: "'Inter Variable', sans-serif",
      [FontFamily.Roboto]: "'Roboto Variable', sans-serif",
      [FontFamily.OpenSans]: "'Open Sans Variable', sans-serif",
      [FontFamily.NotoSans]: "'Noto Sans Variable', sans-serif",
      [FontFamily.SourceSans]: "'Source Sans 3 Variable', sans-serif",
      [FontFamily.UbuntuSans]: "'Ubuntu Sans Variable', sans-serif",
      [FontFamily.Montserrat]: "'Montserrat Variable', sans-serif",
      [FontFamily.RobotoCondensed]: "'Roboto Condensed Variable', sans-serif",
      [FontFamily.Oswald]: "'Oswald Variable', sans-serif",
      [FontFamily.IbmPlexSans]: "'IBM Plex Sans Variable', sans-serif",
      [FontFamily.RobotoMono]: "'Roboto Mono Variable', sans-serif",
    };

    const horizontal = {
      [TextAlignment.Left]: 'flex-start',
      [TextAlignment.Center]: 'center',
      [TextAlignment.Right]: 'flex-end',
    };

    const vertical = {
      [TextAnchor.Top]: 'flex-start',
      [TextAnchor.Middle]: 'center',
      [TextAnchor.Bottom]: 'flex-end',
    };

    return {
      zIndex: style.layer,
      left: String(layout.x) + 'px',
      top: String(layout.y) + 'px',
      width: String(layout.width) + 'px',
      height: String(layout.height) + 'px',
      color: widget.appearance.color,
      background:
        group && widget.group.fillEnabled === false ? 'transparent' : widget.appearance.background,
      borderWidth: '0px',
      borderRadius: String(style.radius) + 'px',
      boxShadow: style.glow ? '0 0 12px 3px ' + style.glowColor : 'none',
      '--widget-opacity': widget.appearance.opacity,
      '--animation-color': style.animationColor,
      '--animation-intensity': style.animationIntensity / 100,
      '--glow-color': style.glowColor,
      opacity: widget.appearance.opacity,
      fontSize: String(widget.appearance.fontSize) + 'px',
      fontFamily: families[style.fontFamily],
      fontWeight: style.bold ? 700 : 400,
      textAlign: style.alignment,
      justifyContent: horizontal[style.alignment],
      alignItems: vertical[style.vertical],
      textShadow: this.textShadow(
        style.outline,
        style.shadow,
        style.outlineWidth,
        style.outlineColor,
      ),
      animationName:
        style.animation === WidgetAnimation.None ? 'none' : 'widget-' + style.animation,
      animationDuration: String(style.periodMs) + 'ms',
      animationIterationCount: 'infinite',
    };
  }

  private textShadow(outline: boolean, shadow: boolean, width: number, color: string): string {
    const values = outline
      ? [-1, 1].flatMap((x) =>
          [-1, 1].map((y) => String(x * width) + 'px ' + String(y * width) + 'px ' + color),
        )
      : [];

    if (shadow === true) {
      values.push('2px 2px 3px #000');
    }

    return values.length === 0 ? 'none' : values.join(', ');
  }
}
