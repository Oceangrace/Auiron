import type { UiSelectConfiguration } from './ui-select-configuration';
import type { UiSelectState } from './ui-select-state';

export class UiSelectController<T extends string> {
  private readonly actions: ReadonlyMap<string, () => void>;
  private search = '';
  private searchTime = 0;
  private readonly observer: ResizeObserver;
  private measuredWidth: number | null = null;
  private revealFrame: number | null = null;
  private readonly onFocus = (event: FocusEvent): void => {
    if (event.target instanceof Node === false) {
      return;
    }

    if (
      this.trigger.contains(event.target) === false &&
      this.popup()?.contains(event.target) !== true
    ) {
      this.close();
    }
  };

  private readonly onOutside = (event: PointerEvent): void => {
    if (event.target instanceof Node === false) {
      return;
    }

    if (
      this.trigger.contains(event.target) === false &&
      this.popup()?.contains(event.target) !== true
    ) {
      this.close();
    }
  };

  private readonly onScroll = (event: Event): void => {
    if (event.target instanceof Node === true && this.popup()?.contains(event.target) === true) {
      return;
    }

    this.close();
  };

  private readonly onDismiss = (): void => {
    this.close();
  };

  public constructor(
    private readonly configuration: UiSelectConfiguration<T>,
    private readonly trigger: HTMLButtonElement,
    private readonly popup: () => HTMLElement | null,
    private readonly state: UiSelectState,
    private readonly onSelect: (value: T) => void,
  ) {
    this.observer = new ResizeObserver(() => {
      this.place();
    });
    this.actions = new Map([
      [
        'ArrowDown',
        () => {
          this.move(1);
        },
      ],
      [
        'ArrowUp',
        () => {
          this.move(-1);
        },
      ],
      [
        'Home',
        () => {
          this.open();
          this.state.activeIndex = 0;
        },
      ],
      [
        'End',
        () => {
          this.open();
          this.state.activeIndex = this.configuration.options.length - 1;
        },
      ],
      [
        'Enter',
        () => {
          this.confirm();
        },
      ],
      [
        ' ',
        () => {
          this.confirm();
        },
      ],
      [
        'Escape',
        () => {
          this.close();
          this.trigger.focus({ preventScroll: true });
        },
      ],
    ]);
  }

  public start(): void {
    document.addEventListener('pointerdown', this.onOutside, true);
    document.addEventListener('focusin', this.onFocus);
    window.addEventListener('scroll', this.onScroll, true);
    window.addEventListener('resize', this.onDismiss);
    window.addEventListener('blur', this.onDismiss);
    this.observer.observe(this.trigger);
  }

  public dispose(): void {
    this.close();
    this.observer.disconnect();

    if (this.revealFrame !== null) {
      cancelAnimationFrame(this.revealFrame);
    }

    document.removeEventListener('pointerdown', this.onOutside, true);
    document.removeEventListener('focusin', this.onFocus);
    window.removeEventListener('scroll', this.onScroll, true);
    window.removeEventListener('resize', this.onDismiss);
    window.removeEventListener('blur', this.onDismiss);
  }

  public toggle(): void {
    if (this.state.open === true) {
      this.close();

      return;
    }

    this.open();
  }

  public close(): void {
    this.state.open = false;
    this.search = '';
  }

  public choose(index: number): void {
    const option = this.configuration.options[index];

    if (option === undefined || this.configuration.disabled === true) {
      return;
    }

    this.close();

    if (option.value !== this.configuration.modelValue) {
      this.onSelect(option.value);
    }

    this.trigger.focus({ preventScroll: true });
  }

  public sync(): void {
    if (this.configuration.disabled === true) {
      this.close();

      return;
    }

    this.state.activeIndex =
      this.configuration.options.length === 0
        ? -1
        : Math.max(
            0,
            this.configuration.options.findIndex(
              (option) => option.value === this.configuration.modelValue,
            ),
          );
    this.place();
  }

  public keydown(event: KeyboardEvent): void {
    if (event.key === 'Tab') {
      this.close();

      return;
    }

    const action = this.actions.get(event.key);

    if (action !== undefined) {
      event.preventDefault();
      event.stopPropagation();
      action();
      this.scheduleReveal();

      return;
    }

    this.findByText(event);
  }

  public reveal(): void {
    const list = this.popup()?.querySelector('.ui-select-list');

    const active = this.popup()?.querySelector('[data-highlighted="true"]');

    if (list instanceof HTMLElement === false || active instanceof HTMLElement === false) {
      return;
    }

    const bounds = list.getBoundingClientRect();

    const row = active.getBoundingClientRect();

    const delta =
      row.top < bounds.top ? row.top - bounds.top : Math.max(0, row.bottom - bounds.bottom);

    list.scrollTop += delta;
  }

  private scheduleReveal(): void {
    if (this.revealFrame !== null) {
      cancelAnimationFrame(this.revealFrame);
    }

    this.revealFrame = requestAnimationFrame(() => {
      this.revealFrame = null;
      this.reveal();
    });
  }

  private open(): void {
    if (
      this.configuration.disabled === true ||
      (this.configuration.remoteSearch !== true &&
        (this.configuration.allOptions ?? this.configuration.options).length === 0)
    ) {
      return;
    }

    this.state.open = true;
    this.measuredWidth = null;
    this.sync();
  }

  private confirm(): void {
    if (this.state.open === false) {
      this.open();

      return;
    }

    this.choose(this.state.activeIndex);
  }

  private move(direction: number): void {
    if (this.state.open === false) {
      this.open();

      return;
    }

    const count = this.configuration.options.length;

    if (count === 0) {
      return;
    }

    this.state.activeIndex = (this.state.activeIndex + direction + count) % count;
  }

  private findByText(event: KeyboardEvent): void {
    if (
      event.key.length !== 1 ||
      event.ctrlKey === true ||
      event.altKey === true ||
      event.metaKey === true
    ) {
      return;
    }

    event.preventDefault();
    this.open();
    const now = Date.now();

    this.search = now - this.searchTime > 700 ? event.key : this.search + event.key;
    this.searchTime = now;
    const index = this.configuration.options.findIndex((option) =>
      option.label.toLowerCase().startsWith(this.search.toLowerCase()),
    );

    if (index !== -1) {
      this.state.activeIndex = index;
    }
  }

  private place(): void {
    if (this.state.open === false) {
      return;
    }

    const rect = this.trigger.getBoundingClientRect();

    const style = getComputedStyle(this.trigger);

    const scale = rect.width / Math.max(1, this.trigger.offsetWidth);

    this.state.fontSize = parseFloat(style.fontSize) * scale;
    const row =
      this.configuration.compact === true ? this.state.fontSize * 1.9 : this.state.fontSize * 2.4;

    const desired =
      (Math.min(8, (this.configuration.allOptions ?? this.configuration.options).length) + 2) *
        row +
      2;

    const below = window.innerHeight - rect.bottom - 12;

    const above = rect.top - 12;

    const upward = below < desired && above > below;

    const height = Math.max(0, Math.min(desired, upward ? above : below));

    this.measuredWidth ??= this.menuWidth(rect.width);
    this.state.width = this.measuredWidth;
    this.state.left = Math.max(8, Math.min(rect.left, window.innerWidth - this.state.width - 8));
    this.state.top = upward ? rect.top - height - 4 : rect.bottom + 4;
    this.state.maxHeight = height;
  }

  private menuWidth(triggerWidth: number): number {
    const style = getComputedStyle(this.trigger);

    const context = document.createElement('canvas').getContext('2d');

    if (context === null) {
      return Math.min(
        Math.max(triggerWidth, this.configuration.popupMinWidth ?? 0),
        window.innerWidth - 16,
      );
    }

    context.font = `${style.fontWeight} ${String(this.state.fontSize)}px ${style.fontFamily}`;
    const measured =
      Math.max(
        0,
        ...(this.configuration.allOptions ?? this.configuration.options).map(
          (option) => context.measureText(option.label).width,
        ),
      ) + 48;

    return Math.min(
      window.innerWidth - 16,
      Math.max(triggerWidth, this.configuration.popupMinWidth ?? 0, Math.min(measured, 480)),
    );
  }
}
