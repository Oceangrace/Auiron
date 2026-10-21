import { shallowReadonly, shallowRef } from 'vue';
import type { ShallowRef } from 'vue';
import type { DesktopCommands } from '../../../core/desktop/desktop-commands';
import type { InterfaceWorkspace } from '../model/interface-workspace';
import type { PreviewContext } from '../model/preview-context';
import type { PreviewVisual } from '../model/preview-visual';
import type { PreviewPlayback } from '../model/preview-playback';
import { PreviewScenario } from '../model/preview-scenario';
import { PreviewSamples } from './preview-samples';
import { HudLocalization } from '../scene/hud-localization';
export class AppearancePreview {
  private static readonly refreshIntervalMs = 100;
  public readonly enabled = shallowRef(false);
  public readonly scenario = shallowRef<PreviewScenario | null>(null);
  public readonly members = shallowRef(5);
  public readonly playing = shallowRef(true);
  private readonly samples = new PreviewSamples();
  private readonly localization = new HudLocalization();
  public readonly context = shallowRef<PreviewContext>({
    active: true,
    remaining: 5,
    stacks: 1,
    progress: 50,
  });

  private readonly visuals = shallowRef<Readonly<Record<string, PreviewVisual>>>({});
  private readonly playback = shallowRef<PreviewPlayback | null>(null);
  public readonly error = shallowRef<string | null>(null);
  private workspace: InterfaceWorkspace | null = null;
  private revision = 0;
  private timer: ReturnType<typeof setTimeout> | null = null;
  private startedAt = 0;
  private pausedAt = 0;
  private disposed = false;
  public constructor(
    private readonly desktop: DesktopCommands,
    private readonly locale: () => string,
  ) {}

  public getVisuals(): Readonly<ShallowRef<Readonly<Record<string, PreviewVisual>>>> {
    return shallowReadonly(this.visuals);
  }

  public getPlayback(): Readonly<ShallowRef<PreviewPlayback | null>> {
    return shallowReadonly(this.playback);
  }

  public sync(workspace: InterfaceWorkspace): void {
    this.workspace = workspace;
    this.schedule();
  }

  public enable(value: boolean): void {
    if (value === true && this.enabled.value === false) {
      this.startedAt = performance.now();
      this.pausedAt = this.startedAt;
    }

    this.enabled.value = value;
    this.schedule();
  }

  public update(patch: Partial<PreviewContext>): void {
    if (this.scenario.value !== null) {
      this.context.value = { ...this.context.value, widgets: {}, instances: {} };
    }

    this.scenario.value = null;
    this.context.value = { ...this.context.value, ...patch };
    this.schedule();
  }

  public sample(scenario: PreviewScenario, members = this.members.value): void {
    if (this.scenario.value !== scenario) {
      this.startedAt = performance.now();
      this.pausedAt = this.startedAt;
    }

    this.scenario.value = scenario;
    this.members.value = Math.max(0, Math.min(5, Math.round(members)));
    this.enable(true);
  }

  public animate(value: boolean): void {
    if (this.playing.value === value) {
      return;
    }

    if (value === false) {
      this.pausedAt = performance.now();
    } else {
      this.startedAt += performance.now() - this.pausedAt;
    }

    this.playing.value = value;
    this.schedule();
  }

  private schedule(): void {
    this.revision++;

    if (this.playback.value !== null) {
      this.playback.value = { ...this.playback.value, running: false };
    }

    if (this.timer !== null) {
      clearTimeout(this.timer);
    }

    this.timer = setTimeout(() => {
      this.timer = null;
      void this.refresh(this.revision);
    }, AppearancePreview.refreshIntervalMs);
  }

  private async refresh(revision: number): Promise<void> {
    if (this.workspace === null || this.disposed === true) {
      return;
    }

    const sampledAt = performance.now();

    try {
      const visuals = await this.evaluate(this.workspace);

      if (revision === this.revision) {
        this.visuals.value = visuals;
        this.playback.value = { sampledAt, running: this.animating() };
        this.error.value = null;
      }
    } catch (error) {
      if (revision === this.revision) {
        this.error.value = String(error);
      }
    } finally {
      this.repeat(revision, sampledAt);
    }
  }

  private evaluate(source: InterfaceWorkspace): Promise<Record<string, PreviewVisual>> {
    let workspace: InterfaceWorkspace = {
      ...source,
      documents: source.documents.map((document) =>
        this.localization.document(document, this.locale()),
      ),
    };

    if (this.enabled.value === true && this.scenario.value !== null) {
      this.context.value = this.samples.context(
        workspace,
        this.scenario.value,
        this.members.value,
        ((this.playing.value ? performance.now() : this.pausedAt) - this.startedAt) / 1000,
      );
      workspace = this.samples.workspace(workspace);
    }

    return this.desktop.invoke<Record<string, PreviewVisual>>('interfaces_preview', {
      workspace,
      context: this.enabled.value
        ? { ...this.context.value, simulating: true }
        : { active: null, remaining: null, stacks: null, progress: null },
    });
  }

  private animating(): boolean {
    return (
      this.enabled.value === true &&
      this.playing.value === true &&
      (this.scenario.value === PreviewScenario.Combat ||
        this.scenario.value === PreviewScenario.Critical)
    );
  }

  private repeat(revision: number, sampledAt: number): void {
    if (revision === this.revision && this.disposed === false && this.animating() === true) {
      this.timer = setTimeout(
        () => {
          this.timer = null;
          void this.refresh(revision);
        },
        Math.max(0, AppearancePreview.refreshIntervalMs - (performance.now() - sampledAt)),
      );
    }
  }

  public dispose(): void {
    this.disposed = true;
    this.revision++;

    if (this.playback.value !== null) {
      this.playback.value = { ...this.playback.value, running: false };
    }

    if (this.timer !== null) {
      clearTimeout(this.timer);
    }
  }
}
