import { EditorAction } from '../model/editor-action';

export class EditorShortcuts {
  public constructor(private readonly execute: (action: EditorAction) => void) {}

  public handle(event: KeyboardEvent): void {
    if (event.defaultPrevented === true || document.querySelector('dialog[open]') !== null) {
      return;
    }

    if (event.altKey === true) {
      return;
    }

    if (this.saveShortcut(event) === true) {
      return;
    }

    if (this.isTextInput(event.target) === true) {
      return;
    }

    const action = this.editAction(event);

    if (action !== null) {
      event.preventDefault();
      this.execute(action);
    }
  }

  private editAction(event: KeyboardEvent): EditorAction | null {
    const shortcut = event.ctrlKey === true;

    if (shortcut === true) {
      return this.commandAction(event);
    }

    const arrows = new Map([
      ['ArrowLeft', event.shiftKey ? EditorAction.LeftFast : EditorAction.Left],
      ['ArrowRight', event.shiftKey ? EditorAction.RightFast : EditorAction.Right],
      ['ArrowUp', event.shiftKey ? EditorAction.UpFast : EditorAction.Up],
      ['ArrowDown', event.shiftKey ? EditorAction.DownFast : EditorAction.Down],
      ['Escape', EditorAction.Deselect],
      ['Delete', EditorAction.Remove],
    ]);

    return arrows.get(event.key) ?? null;
  }

  private commandAction(event: KeyboardEvent): EditorAction | null {
    if (this.matches(event, 'KeyZ', ['z', 'я']) === true) {
      return event.shiftKey ? EditorAction.Redo : EditorAction.Undo;
    }

    if (this.matches(event, 'KeyY', ['y', 'н']) === true) {
      return EditorAction.Redo;
    }

    return this.matches(event, 'KeyD', ['d', 'в']) ? EditorAction.Duplicate : null;
  }

  private saveShortcut(event: KeyboardEvent): boolean {
    const modifier = event.ctrlKey === true;

    if (modifier === false || this.matches(event, 'KeyS', ['s', 'ы']) === false) {
      return false;
    }

    event.preventDefault();

    this.execute(EditorAction.Save);

    return true;
  }

  private matches(event: KeyboardEvent, code: string, keys: readonly string[]): boolean {
    return event.code === code || keys.includes(event.key.toLowerCase());
  }

  private isTextInput(target: EventTarget | null): boolean {
    if (target instanceof HTMLElement === false) {
      return false;
    }

    return (
      target.closest(
        'input, textarea, select, [role="combobox"], [role="listbox"], [contenteditable]',
      ) !== null
    );
  }
}
