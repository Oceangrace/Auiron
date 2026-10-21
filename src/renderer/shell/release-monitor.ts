import { shallowReadonly, shallowRef } from 'vue';
import type { ShallowRef } from 'vue';
import type { DesktopCommands } from '../../core/desktop/desktop-commands';
import type { ReleaseInfo } from '../../core/updates/release-info';
import type { ReleaseState } from './release-state';

export class ReleaseMonitor {
  private static readonly INTERVAL_MS = 4 * 60 * 60 * 1000;
  private readonly state = shallowRef<ReleaseState>({
    currentVersion: null,
    info: null,
    checking: false,
    checkFailed: false,
    openFailed: false,
    dismissedVersion: null,
  });

  private timer: ReturnType<typeof setInterval> | null = null;
  private running = false;

  public constructor(private readonly desktop: DesktopCommands) {}

  public getState(): Readonly<ShallowRef<ReleaseState>> {
    return shallowReadonly(this.state);
  }

  public start(): void {
    if (this.running === true) {
      return;
    }

    this.running = true;
    void this.loadVersion();
    void this.check();
    this.timer = setInterval(() => {
      void this.check();
    }, ReleaseMonitor.INTERVAL_MS);
  }

  private async loadVersion(): Promise<void> {
    try {
      const currentVersion = await this.desktop.invoke<string>('app_version');

      if (this.running === true) {
        this.state.value = { ...this.state.value, currentVersion };
      }
    } catch {
      // A successful release check also supplies the installed version.
    }
  }

  public async check(): Promise<void> {
    if (this.canCheck() === false) {
      return;
    }

    this.state.value = { ...this.state.value, checking: true, checkFailed: false };

    try {
      const info = await this.desktop.invoke<ReleaseInfo>('app_release_check');

      if (this.running === true) {
        this.state.value = {
          ...this.state.value,
          currentVersion: info.currentVersion,
          info,
          checking: false,
        };
      }
    } catch {
      if (this.running === true) {
        this.state.value = { ...this.state.value, checking: false, checkFailed: true };
      }
    }
  }

  private canCheck(): boolean {
    return this.running === true && this.state.value.checking === false;
  }

  public async open(): Promise<void> {
    this.state.value = { ...this.state.value, openFailed: false };

    try {
      await this.desktop.invoke('app_release_open');
    } catch {
      if (this.running === true) {
        this.state.value = { ...this.state.value, openFailed: true };
      }
    }
  }

  public dismiss(): void {
    this.state.value = {
      ...this.state.value,
      dismissedVersion: this.state.value.info?.latestVersion ?? null,
    };
  }

  public dispose(): void {
    this.running = false;

    if (this.timer !== null) {
      clearInterval(this.timer);
      this.timer = null;
    }
  }
}
