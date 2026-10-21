import { PreviewScenario } from '../model/preview-scenario';

export class PreviewCombat {
  private static readonly cycleSeconds = 16;
  private static readonly healPercent = 9;
  private static readonly tickSeconds = [7, 9, 11, 13];

  public percent(base: number | null, elapsed: number, index: number): number | null {
    if (base === null || base === 0) {
      return base;
    }

    const phase = this.phase(elapsed, index);

    const damage = Number(phase >= 2) * 12 + Number(phase >= 4) * 16;

    const healing = PreviewCombat.tickSeconds.filter((tick) => tick <= phase).length;

    return Math.max(1, Math.min(100, base - damage + healing * PreviewCombat.healPercent));
  }

  public healing(maximum: number, elapsed: number, index: number): number {
    const phase = this.phase(elapsed, index);

    return phase < 5
      ? 0
      : PreviewCombat.tickSeconds.filter((tick) => tick > phase).length *
          maximum *
          (PreviewCombat.healPercent / 100);
  }

  public phase(elapsed: number, index = 0): number {
    return (elapsed + index * 1.6) % PreviewCombat.cycleSeconds;
  }

  public running(scenario: PreviewScenario): boolean {
    return scenario === PreviewScenario.Combat || scenario === PreviewScenario.Critical;
  }

  public timing(
    scenario: PreviewScenario,
    elapsed: number,
    index: number,
  ): Record<string, number | boolean | null> {
    if (this.running(scenario) === false) {
      return {
        active: false,
        remaining: null,
        duration: null,
        stacks: null,
        elapsed: null,
        countdown: null,
      };
    }

    const duration = scenario === PreviewScenario.Critical ? 4000 : 12000 + index * 2000;

    const age = ((elapsed + index * 2.3) * 1000) % (duration + 4000);

    const remaining = Math.max(0, duration - age);

    return {
      active: remaining > 0,
      remaining,
      duration,
      stacks: 1 + (Math.floor(elapsed / 3) % 3),
      elapsed: age,
      countdown: remaining,
    };
  }
}
