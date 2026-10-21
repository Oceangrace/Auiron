import type { HealingPrediction } from '../model/healing-prediction';

export class BarHealingLayout {
  public loss(
    value: number,
    vertical: boolean,
    edge: string,
    inverted: boolean,
  ): Readonly<Record<string, string>> {
    const current = Math.max(0, Math.min(100, value));

    const extent = String(100 - current) + '%';

    return {
      [edge]: String(inverted ? 0 : current) + '%',
      width: vertical ? '100%' : extent,
      height: vertical ? extent : '100%',
    };
  }

  public style(
    prediction: HealingPrediction,
    vertical: boolean,
    edge: string,
    inverted: boolean,
    excess: boolean,
  ): Readonly<Record<string, string>> {
    const from = excess
      ? Math.max(0, prediction.startPercent - prediction.overhealPercent)
      : prediction.startPercent;

    const to = excess ? prediction.startPercent : prediction.endPercent;

    const start = inverted ? 100 - to : from;

    const extent = String(to - from) + '%';

    return {
      [edge]: String(start) + '%',
      width: vertical ? '100%' : extent,
      height: vertical ? extent : '100%',
    };
  }
}
