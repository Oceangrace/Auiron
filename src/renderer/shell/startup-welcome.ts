import { shallowReadonly, shallowRef } from 'vue';
import type { ShallowRef } from 'vue';
import { WelcomeStage } from './welcome-stage';

export class StartupWelcome {
  private readonly stage = shallowRef(WelcomeStage.Window);
  private readonly guide = shallowRef(false);
  private attached = false;

  public state(): Readonly<ShallowRef<WelcomeStage>> {
    return shallowReadonly(this.stage);
  }

  public guideRequested(): Readonly<ShallowRef<boolean>> {
    return shallowReadonly(this.guide);
  }

  public observe(attached: boolean | null): void {
    this.attached = attached === true;

    if (attached === true) {
      this.dismiss();
    }
  }

  public dismiss(): void {
    this.stage.value = WelcomeStage.Hidden;
    this.guide.value = false;
  }

  public collapse(): void {
    this.stage.value = this.attached ? WelcomeStage.Hidden : WelcomeStage.Notice;
    this.guide.value = false;
  }

  public openGuide(): void {
    this.guide.value = true;
    this.stage.value = WelcomeStage.Window;
  }
}
