import type { InterfaceDocument } from '../model/interface-document';

export class EditorHistory {
  private readonly undoStack: (readonly InterfaceDocument[])[] = [];
  private readonly redoStack: (readonly InterfaceDocument[])[] = [];

  public record(documents: readonly InterfaceDocument[]): void {
    this.undoStack.push(documents);
    this.undoStack.splice(0, Math.max(0, this.undoStack.length - 100));
    this.redoStack.length = 0;
  }

  public undo(current: readonly InterfaceDocument[]): readonly InterfaceDocument[] | null {
    const previous = this.undoStack.pop();

    if (previous === undefined) {
      return null;
    }

    this.redoStack.push(current);

    return previous;
  }

  public redo(current: readonly InterfaceDocument[]): readonly InterfaceDocument[] | null {
    const next = this.redoStack.pop();

    if (next === undefined) {
      return null;
    }

    this.undoStack.push(current);

    return next;
  }

  public canUndo(): boolean {
    return this.undoStack.length !== 0;
  }

  public canRedo(): boolean {
    return this.redoStack.length !== 0;
  }
}
