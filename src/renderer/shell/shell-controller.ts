import { shallowRef, shallowReadonly } from 'vue';
import type { ShallowRef } from 'vue';
import type { DesktopCommands } from '../../core/desktop/desktop-commands';
import type { OverlayPreferences } from '../../core/preferences/overlay-preferences';
import type { PanelLayout } from '../../core/preferences/panel-layout';
import { OverlayAnchor } from '../../core/preferences/overlay-anchor';
import { ToolbarLayout } from '../../core/preferences/toolbar-layout';
import { ScreenPreset } from '../../core/preferences/screen-preset';
import { ClassPalette } from '../../core/preferences/class-palette';
export class ShellController {
  private readonly state = shallowRef<OverlayPreferences>({
    classColors: {},
    locale: '',
    screenPreset: ScreenPreset.Auto,
    uiScale: 100,
    backgroundTransparency: 0,
    toolbarAnchor: OverlayAnchor.TopCenter,
    toolbarLayout: ToolbarLayout.Horizontal,
    toolbarPosition: null,
    toolbarShowClock: true,
    toolbarShowPlayTime: false,
    toolbarAutoHide: false,
    settingsPanel: null,
  });

  private readonly error = shallowRef<string | null>(null);
  private pending: Promise<void> = Promise.resolve();
  private panelTimer: ReturnType<typeof setTimeout> | null = null;
  private readOnly = false;
  public constructor(
    private readonly desktop: DesktopCommands,
    defaultLocale: string,
  ) {
    this.state.value = { ...this.state.value, locale: defaultLocale };
  }

  public getState(): Readonly<ShallowRef<OverlayPreferences>> {
    return shallowReadonly(this.state);
  }

  public getError(): Readonly<ShallowRef<string | null>> {
    return shallowReadonly(this.error);
  }

  public async start(): Promise<void> {
    try {
      this.state.value = await this.desktop.invoke<OverlayPreferences>('preferences_read');
    } catch (error) {
      this.error.value = String(error);
      this.readOnly = true;
    }
  }

  public preview(changes: Partial<OverlayPreferences>): void {
    if (this.readOnly === false) {
      this.state.value = { ...this.state.value, ...changes };
    }
  }

  public update(changes: Partial<OverlayPreferences>): void {
    if (this.readOnly === true) {
      return;
    }

    const next = { ...this.state.value, ...changes };

    this.state.value = next;
    this.pending = this.pending
      .then(async () => {
        await this.desktop.invoke('preferences_save', { preferences: next });
        this.error.value = null;
      })
      .catch((error: unknown) => {
        this.error.value = String(error);
      });
  }

  public async minimize(): Promise<void> {
    try {
      await this.desktop.invoke('app_minimize');
    } catch (error) {
      this.error.value = String(error);
    }
  }

  public saveSettingsPanel(layout: PanelLayout): void {
    this.preview({ settingsPanel: layout });

    if (this.panelTimer !== null) {
      clearTimeout(this.panelTimer);
    }

    this.panelTimer = setTimeout(() => {
      this.flushPanel();
    }, 200);
  }

  private flushPanel(): void {
    if (this.panelTimer === null) {
      return;
    }

    clearTimeout(this.panelTimer);
    this.panelTimer = null;
    this.update({ settingsPanel: this.state.value.settingsPanel });
  }

  public apply(root: HTMLElement, height: number): void {
    new ClassPalette().apply(root, this.state.value.classColors);

    const scales = new Map([
      [ScreenPreset.HD, 0.85],
      [ScreenPreset.FullHD, 1],
      [ScreenPreset.QHD, 1.3],
    ]);

    const automatic =
      [
        { height: 900, scale: 0.85 },
        { height: 1260, scale: 1 },
        { height: Infinity, scale: 1.3 },
      ].find((preset) => height <= preset.height)?.scale ?? 1.3;

    const scale =
      ((scales.get(this.state.value.screenPreset) ?? automatic) * this.state.value.uiScale) / 100;

    root.style.setProperty('--shell-ui-scale', String(scale));
    root.style.setProperty(
      '--prism-panel-alpha',
      String(1 - this.state.value.backgroundTransparency / 100),
    );
  }

  public async flush(): Promise<void> {
    this.flushPanel();
    await this.pending;
  }
}
