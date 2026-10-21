export class EditorInput {
  public commit(): void {
    const active = document.activeElement;

    if (active instanceof HTMLElement === true) {
      active.blur();
    }
  }
}
