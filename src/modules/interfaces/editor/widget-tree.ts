import { GroupScale } from '../scene/group-scale';
import type { PreviewVisual } from '../model/preview-visual';
import type { WidgetNode } from '../model/widget-node';
import type { WidgetLayout } from '../model/widget-layout';
import { WidgetKind } from '../model/widget-kind';

import { ChildLayout } from '../scene/child-layout';
import { defaultWidgetAnchors } from '../model/widget-anchors';

export class WidgetTree {
  private visuals: Readonly<Record<string, PreviewVisual>> = {};
  public setVisuals(visuals: Readonly<Record<string, PreviewVisual>>): void {
    this.visuals = visuals;
  }

  public isGroup(node: WidgetNode): boolean {
    return node.kind === WidgetKind.Group || node.kind === WidgetKind.DynamicGroup;
  }

  public find(nodes: readonly WidgetNode[], id: string | null): WidgetNode | null {
    for (const node of nodes) {
      if (node.id === id) {
        return node;
      }

      const found = this.find(node.children, id);

      if (found !== null) {
        return found;
      }
    }

    return null;
  }

  public creationParent(nodes: readonly WidgetNode[], id: string | null): string | null {
    const node = this.find(nodes, id);

    if (node === null) {
      return null;
    }

    return this.isGroup(node) ? node.id : (this.parent(nodes, node.id)?.id ?? null);
  }

  public path(nodes: readonly WidgetNode[], id: string): readonly WidgetNode[] {
    for (const node of nodes) {
      if (node.id === id) {
        return [node];
      }

      const children = this.path(node.children, id);

      if (children.length !== 0) {
        return [node, ...children];
      }
    }

    return [];
  }

  public parent(nodes: readonly WidgetNode[], id: string): WidgetNode | null {
    for (const node of nodes) {
      if (node.children.some((child) => child.id === id) === true) {
        return node;
      }

      const found = this.parent(node.children, id);

      if (found !== null) {
        return found;
      }
    }

    return null;
  }

  public update(
    nodes: readonly WidgetNode[],
    id: string,
    change: (node: WidgetNode) => WidgetNode,
  ): readonly WidgetNode[] {
    return nodes.map((node) =>
      node.id === id ? change(node) : { ...node, children: this.update(node.children, id, change) },
    );
  }

  public remove(nodes: readonly WidgetNode[], id: string): readonly WidgetNode[] {
    return nodes
      .filter((node) => node.id !== id)
      .map((node) => ({ ...node, children: this.remove(node.children, id) }));
  }

  public append(
    nodes: readonly WidgetNode[],
    parentId: string | null,
    widget: WidgetNode,
  ): readonly WidgetNode[] {
    if (parentId === null) {
      return [...nodes, widget];
    }

    return this.update(nodes, parentId, (parent) => ({
      ...parent,
      children: [...parent.children, widget],
    }));
  }

  public world(nodes: readonly WidgetNode[], id: string): WidgetLayout | null {
    const node = this.find(nodes, id);

    if (node === null) {
      return null;
    }

    const parent = this.parent(nodes, id);

    if (parent === null) {
      return new GroupScale().frame(node);
    }

    const origin = this.world(nodes, parent.id);

    if (origin === null) {
      return new GroupScale().frame(node);
    }

    const factor = this.inheritedScale(nodes, id);

    const local = { ...origin, width: origin.width / factor, height: origin.height / factor };

    const layout = new ChildLayout().resolve(node, parent, local, true, this.visuals);

    const frame = new GroupScale().frame(node, layout);

    return {
      x: origin.x + layout.x * factor,
      y: origin.y + layout.y * factor,
      width: frame.width * factor,
      height: frame.height * factor,
    };
  }

  public inheritedScale(nodes: readonly WidgetNode[], id: string): number {
    return this.path(nodes, id)
      .slice(0, -1)
      .reduce((factor, node) => factor * new GroupScale().factor(node), 1);
  }

  public reorder(
    nodes: readonly WidgetNode[],
    id: string,
    direction: number,
  ): readonly WidgetNode[] {
    const index = nodes.findIndex((node) => node.id === id);

    if (index === -1) {
      return nodes.map((node) => ({
        ...node,
        children: this.reorder(node.children, id, direction),
      }));
    }

    const next = Math.max(0, Math.min(nodes.length - 1, index + direction));

    const widget = nodes[index];

    if (widget === undefined) {
      return nodes;
    }

    const reordered = [...nodes];

    reordered.splice(index, 1);
    reordered.splice(next, 0, widget);

    return reordered;
  }

  public canReparent(
    nodes: readonly WidgetNode[],
    widget: WidgetNode,
    parentId: string | null,
  ): boolean {
    if (parentId === widget.id || this.find(widget.children, parentId) !== null) {
      return false;
    }

    const parent = this.find(nodes, parentId);

    return parentId === null || (parent !== null && this.isGroup(parent));
  }

  public flatten(nodes: readonly WidgetNode[]): readonly WidgetNode[] {
    return nodes.flatMap((node) => [node, ...this.flatten(node.children)]);
  }

  public sameLayout(left: WidgetLayout, right: WidgetLayout): boolean {
    return (
      left.x === right.x &&
      left.y === right.y &&
      left.width === right.width &&
      left.height === right.height
    );
  }

  private parentContentOrigin(
    nodes: readonly WidgetNode[],
    parentId: string | null,
  ): { x: number; y: number } {
    const layout = this.world(nodes, parentId ?? '');

    const parent = this.find(nodes, parentId);

    if (parent === null || layout === null) {
      return { x: 0, y: 0 };
    }

    const factor = this.inheritedScale(nodes, parent.id) * new GroupScale().factor(parent);

    const content = new ChildLayout().content(parent, {
      ...layout,
      width: layout.width / factor,
      height: layout.height / factor,
    });

    return { x: layout.x + content.x * factor, y: layout.y + content.y * factor };
  }

  public relocated(
    nodes: readonly WidgetNode[],
    widget: WidgetNode,
    parentId: string | null,
  ): WidgetNode {
    const origin = this.world(nodes, widget.id) ?? widget.layout;

    const parentOrigin = this.parentContentOrigin(nodes, parentId);

    const parent = this.find(nodes, parentId);

    const factor =
      parent === null ? 1 : this.inheritedScale(nodes, parent.id) * new GroupScale().factor(parent);

    const own = new GroupScale().factor(widget);

    return {
      ...widget,
      anchors: { ...defaultWidgetAnchors },
      layout: {
        ...origin,
        x: (origin.x - parentOrigin.x) / factor,
        y: (origin.y - parentOrigin.y) / factor,
        width: origin.width / factor / own,
        height: origin.height / factor / own,
      },
    };
  }
}
