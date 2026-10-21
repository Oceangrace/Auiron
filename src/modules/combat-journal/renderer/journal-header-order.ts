import type { JournalHeaderButton } from '../model/journal-header-button';

export class JournalHeaderOrder {
  public selected: JournalHeaderButton | null = null;
  public draft: readonly JournalHeaderButton[] | null = null;
  public dragging = false;
  private pointer: { id: number; x: number; element: HTMLElement } | null = null;

  public constructor(
    private readonly read: () => readonly JournalHeaderButton[],
    private readonly change: (buttons: readonly JournalHeaderButton[]) => void,
  ) {}

  public move(button: JournalHeaderButton, offset: number): void {
    this.selected = button;
    const buttons = [...this.read()];

    const index = buttons.indexOf(button);

    const target = index + offset;

    if (index === -1 || target < 0 === true || target >= buttons.length === true) {
      return;
    }

    buttons.splice(index, 1);
    buttons.splice(target, 0, button);
    this.change(buttons);
  }

  public begin(event: PointerEvent, button: JournalHeaderButton): void {
    if (event.button !== 0 || event.currentTarget instanceof HTMLElement === false) {
      return;
    }

    this.selected = button;
    const element = event.currentTarget.parentElement;

    if (element === null) {
      return;
    }

    this.pointer = { id: event.pointerId, x: event.clientX, element };
    element.setPointerCapture(event.pointerId);
  }

  public drag(event: PointerEvent): void {
    if (this.pointer?.id !== event.pointerId || this.selected === null) {
      return;
    }

    if (this.dragging === false && Math.abs(event.clientX - this.pointer.x) < 5 === true) {
      return;
    }

    this.dragging = true;
    this.reorder(event, this.selected);
  }

  private reorder(event: PointerEvent, selected: JournalHeaderButton): void {
    const target = document
      .elementFromPoint(event.clientX, event.clientY)
      ?.closest<HTMLElement>('[data-header-button]');

    if (target === null || target === undefined || target.parentElement !== this.pointer?.element) {
      return;
    }

    const buttons = [...(this.draft ?? this.read())];

    const index = buttons.findIndex((button) => button === target.dataset['headerButton']);

    if (index === -1) {
      return;
    }

    buttons.splice(buttons.indexOf(selected), 1);
    buttons.splice(index, 0, selected);
    this.draft = buttons;
  }

  public finish(): void {
    const draft = this.draft;

    this.cancel();

    if (draft !== null) {
      this.change(draft);
    }
  }

  public cancel(): void {
    const pointer = this.pointer;

    this.pointer = null;
    this.draft = null;
    this.dragging = false;

    if (pointer?.element.hasPointerCapture(pointer.id) === true) {
      pointer.element.releasePointerCapture(pointer.id);
    }
  }
}
