import { shallowRef, shallowReadonly } from 'vue';
import type { ShallowRef } from 'vue';

export class ToolbarVisibility {
  private static readonly HIDE_DELAY_MS = 650;
  private readonly expanded = shallowRef(false);
  private timer: ReturnType<typeof setTimeout> | null = null;
  private keyboard = false;

  public state(): Readonly<ShallowRef<boolean>> {
    return shallowReadonly(this.expanded);
  }

  public reveal(): void {
    this.cancel();
    this.expanded.value = true;
  }

  public leave(): void {
    this.cancel();

    if (this.keyboard === false) {
      this.timer = setTimeout(() => {
        this.expanded.value = false;
        this.timer = null;
      }, ToolbarVisibility.HIDE_DELAY_MS);
    }
  }

  public focus(event: FocusEvent): void {
    if (
      event.target instanceof Element === true &&
      event.target.matches(':focus-visible') === true
    ) {
      this.keyboard = true;
      this.reveal();
    }
  }

  public blur(event: FocusEvent): void {
    if (
      event.currentTarget instanceof Element === true &&
      event.relatedTarget instanceof Node === true &&
      event.currentTarget.contains(event.relatedTarget) === true
    ) {
      return;
    }

    this.keyboard = false;
    this.leave();
  }

  public hide(): void {
    this.cancel();
    this.keyboard = false;
    this.expanded.value = false;
  }

  public dispose(): void {
    this.cancel();
  }

  private cancel(): void {
    if (this.timer !== null) {
      clearTimeout(this.timer);
      this.timer = null;
    }
  }
}
