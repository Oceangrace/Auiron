import type { InterfaceDocument } from '../model/interface-document';
import type { Viewport } from '../model/viewport';
import type { WidgetLayout } from '../model/widget-layout';
import type { WidgetNode } from '../model/widget-node';
import { WidgetKind } from '../model/widget-kind';
import { AnchorMode } from '../model/anchor-mode';
import { HudScale } from '../scene/hud-scale';
import type { SelectionMoveEntry } from './selection-move-entry';
import type { WidgetTree } from './widget-tree';

export class SelectionMovement {
  private readonly entries: readonly SelectionMoveEntry[];

  public constructor(
    private readonly document: InterfaceDocument,
    ids: readonly string[],
    private readonly tree: WidgetTree,
    private readonly viewport: Viewport,
  ) {
    this.entries = ids.flatMap((id) => this.entry(id, ids));
  }

  private entry(id: string, ids: readonly string[]): readonly SelectionMoveEntry[] {
    const path = this.tree.path(this.document.roots, id);

    const node = path.at(-1);

    const root = path[0];

    const world = this.tree.world(this.document.roots, id);

    if (node === undefined || root === undefined || world === null) {
      return [];
    }

    if (this.movable(path, ids) === false) {
      return [];
    }

    const hud = new HudScale(this.document.display, this.viewport);

    const screen = hud.screen(root.layout, world);

    const factor = this.tree.inheritedScale(this.document.roots, id);

    // Moving a root also moves its HUD origin; a child only moves within that origin.
    const stepX = root === node ? { ...root.layout, x: root.layout.x + 1 } : root.layout;

    const stepY = root === node ? { ...root.layout, y: root.layout.y + 1 } : root.layout;

    return [
      {
        node,
        screen,
        scaleX: hud.screen(stepX, { ...world, x: world.x + factor }).x - screen.x,
        scaleY: hud.screen(stepY, { ...world, y: world.y + factor }).y - screen.y,
      },
    ];
  }

  private movable(path: readonly WidgetNode[], ids: readonly string[]): boolean {
    return (
      path.at(-1)?.group.locked !== true &&
      path.at(-2)?.kind !== WidgetKind.DynamicGroup &&
      path.slice(0, -1).some((parent) => ids.includes(parent.id)) === false
    );
  }

  public drag(id: string, layout: WidgetLayout, scale: number): readonly WidgetNode[] {
    const node = this.tree.find(this.document.roots, id);

    if (node === null) {
      return this.document.roots;
    }

    return this.move((layout.x - node.layout.x) * scale, (layout.y - node.layout.y) * scale);
  }

  public move(x: number, y: number): readonly WidgetNode[] {
    if (this.entries.length === 0) {
      return this.document.roots;
    }

    const dx = this.clamp(x, 'x', 'width', this.viewport.width);

    const dy = this.clamp(y, 'y', 'height', this.viewport.height);

    return this.entries.reduce<readonly WidgetNode[]>(
      (roots, entry) =>
        this.tree.update(roots, entry.node.id, (node) => ({
          ...node,
          anchors: {
            ...entry.node.anchors,
            right:
              entry.node.anchors.right -
              (entry.node.anchors.horizontal === AnchorMode.Stretch ? dx / entry.scaleX : 0),
            bottom:
              entry.node.anchors.bottom -
              (entry.node.anchors.vertical === AnchorMode.Stretch ? dy / entry.scaleY : 0),
          },
          layout: {
            ...entry.node.layout,
            x: entry.node.layout.x + dx / entry.scaleX,
            y: entry.node.layout.y + dy / entry.scaleY,
          },
        })),
      this.document.roots,
    );
  }

  private clamp(delta: number, axis: 'x' | 'y', size: 'width' | 'height', limit: number): number {
    const lower = Math.max(...this.entries.map((entry) => -entry.screen[axis]));

    const upper = Math.min(
      ...this.entries.map((entry) => limit - entry.screen[axis] - entry.screen[size]),
    );

    // An oversized selection cannot fit; keep its spacing and permit movement toward the screen.
    return Math.max(Math.min(0, lower), Math.min(delta, Math.max(0, upper)));
  }
}
