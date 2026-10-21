import { BarTexture } from '../model/bar-texture';

export class BarSurfaceStyle {
  public texture(kind: BarTexture, size: number, color: string): Readonly<Record<string, string>> {
    const spacing = String(size) + 'px';

    let image = 'none';

    if (kind === BarTexture.Grid) {
      image =
        'linear-gradient(to right, #eaffef80 1px, transparent 1px), ' +
        'linear-gradient(to bottom, #eaffef80 1px, transparent 1px)';
    }

    if (kind === BarTexture.Diagonal) {
      image = 'repeating-linear-gradient(135deg, #eaffef80 0 1px, transparent 1px ' + spacing + ')';
    }

    return {
      backgroundColor: color,
      backgroundImage: image,
      backgroundSize: kind === BarTexture.Grid ? spacing + ' ' + spacing : 'auto',
    };
  }
}
