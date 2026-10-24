import type { DesktopCommands } from '../../core/desktop/desktop-commands';

const INTERACTIVE = '[data-overlay-interactive]';

export class SurfaceController {
  private frame: number | null = null;
  private observed = new Set<Element>();
  private signature = '';
  private readonly moving = new Set<Element>();
  private readonly transitionStart = (event: TransitionEvent): void => {
    if (
      event.propertyName === 'transform' &&
      event.target instanceof Element === true &&
      this.touches(event.target) === true
    ) {
      this.moving.add(event.target);
      this.schedule();
    }
  };

  private readonly transitionEnd = (event: TransitionEvent): void => {
    if (event.target instanceof Element === true && event.propertyName === 'transform') {
      this.moving.delete(event.target);
    }

    this.schedule();
  };

  private fallback: ReturnType<typeof setInterval> | null = null;

  private readonly mutations = new MutationObserver((records) => {
    if (records.some((record) => this.relevant(record)) === true) {
      this.schedule();
    }
  });

  private readonly resizes = new ResizeObserver(() => {
    this.schedule();
  });

  private readonly capture = (event: PointerEvent): void => {
    if (
      event.button === 0 &&
      event.target instanceof Element === true &&
      event.target.closest(INTERACTIVE) !== null
    ) {
      this.setCaptured(true);
    }
  };

  private readonly release = (): void => {
    this.setCaptured(false);
  };

  private readonly schedule = (): void => {
    this.frame ??= requestAnimationFrame(this.observe);
  };

  private readonly observe = (): void => {
    this.frame = null;

    const elements = Array.from(this.element.querySelectorAll(INTERACTIVE));

    this.track(new Set(elements));

    const regions = elements
      .filter((element) => element.checkVisibility())
      .map((element) => {
        const rect = element.getBoundingClientRect();

        return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
      })
      .filter((rect) => rect.width > 0 && rect.height > 0);

    const signature = JSON.stringify(regions);

    if (signature !== this.signature) {
      this.signature = signature;
      void this.desktop.invoke('overlay_regions', { regions }).catch(this.report);
    }

    for (const element of this.moving) {
      if (element.isConnected === false || element.getAnimations().length === 0) {
        this.moving.delete(element);
      }
    }

    if (this.moving.size > 0 === true) {
      this.schedule();
    }
  };

  public constructor(
    private readonly element: HTMLElement,
    private readonly desktop: DesktopCommands,
    private readonly report: (error: unknown) => void,
  ) {}

  public start(): void {
    this.element.addEventListener('pointerdown', this.capture, true);
    this.element.addEventListener('transitionrun', this.transitionStart, true);
    this.element.addEventListener('transitionend', this.transitionEnd, true);
    this.element.addEventListener('transitioncancel', this.transitionEnd, true);
    this.element.addEventListener('animationend', this.schedule, true);
    // Widgets stop bubbling pointer events; release before their handlers run.
    window.addEventListener('pointerup', this.release, true);
    window.addEventListener('pointercancel', this.release, true);
    window.addEventListener('lostpointercapture', this.release, true);
    window.addEventListener('blur', this.release);
    window.addEventListener('resize', this.schedule);
    this.mutations.observe(this.element, {
      subtree: true,
      childList: true,
      attributes: true,
      attributeFilter: ['style', 'class', 'hidden', 'open', 'data-overlay-interactive'],
    });
    this.fallback = setInterval(this.schedule, 1000);
    this.observe();
  }

  public dispose(): void {
    if (this.frame !== null) {
      cancelAnimationFrame(this.frame);
    }

    if (this.fallback !== null) {
      clearInterval(this.fallback);
    }

    this.mutations.disconnect();
    this.resizes.disconnect();
    this.observed.clear();
    this.moving.clear();
    this.element.removeEventListener('pointerdown', this.capture, true);
    this.element.removeEventListener('transitionrun', this.transitionStart, true);
    this.element.removeEventListener('transitionend', this.transitionEnd, true);
    this.element.removeEventListener('transitioncancel', this.transitionEnd, true);
    this.element.removeEventListener('animationend', this.schedule, true);
    window.removeEventListener('pointerup', this.release, true);
    window.removeEventListener('pointercancel', this.release, true);
    window.removeEventListener('lostpointercapture', this.release, true);
    window.removeEventListener('blur', this.release);
    window.removeEventListener('resize', this.schedule);
    this.setCaptured(false);
  }

  // Observing reports once immediately, so only new elements are added: re-observing every
  // scan would turn back into a scan per frame.
  private track(elements: Set<Element>): void {
    for (const element of this.observed) {
      if (elements.has(element) === false) {
        this.resizes.unobserve(element);
      }
    }

    for (const element of elements) {
      if (this.observed.has(element) === false) {
        this.resizes.observe(element);
      }
    }

    this.observed = elements;
  }

  private relevant(record: MutationRecord): boolean {
    if (record.type === 'attributes') {
      return (
        record.attributeName === 'data-overlay-interactive' ||
        (record.target instanceof Element && this.touches(record.target))
      );
    }

    return [...record.addedNodes, ...record.removedNodes].some(
      (node) => node instanceof Element && this.touches(node),
    );
  }

  private touches(element: Element): boolean {
    return (
      element.closest(INTERACTIVE) !== null ||
      element.matches(INTERACTIVE) === true ||
      element.querySelector(INTERACTIVE) !== null
    );
  }

  private setCaptured(captured: boolean): void {
    void this.desktop.invoke('overlay_capture', { captured }).catch(this.report);
  }
}
