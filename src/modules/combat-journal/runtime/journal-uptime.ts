import type { JournalSegment } from '../model/journal-segment';
import type { JournalEffectSpan } from '../model/journal-effect-span';
import type { JournalEffect } from '../model/journal-effect';
import type { JournalUptimeRow } from '../model/journal-uptime-row';

export class JournalUptime {
  public rows(segment: JournalSegment, unit: number): readonly JournalUptimeRow[] {
    const spans = new Map<number, JournalEffectSpan>();

    const finished: JournalEffectSpan[] = [];

    for (const effect of (segment.effects ?? [])
      .filter((entry) => entry.unitId === unit)
      .toSorted((a, b) => (a.atUs === b.atUs ? a.sequence - b.sequence : a.atUs - b.atUs))) {
      this.observe(effect, segment, spans, finished);
    }

    const groups = new Map<string, JournalEffectSpan[]>();

    for (const span of [...finished, ...spans.values()]) {
      this.add(groups, span);
    }

    return [...groups.values()]
      .map((spans) => this.row(spans))
      .sort((a, b) => b.milliseconds - a.milliseconds);
  }

  private observe(
    effect: JournalEffect,
    segment: JournalSegment,
    spans: Map<number, JournalEffectSpan>,
    finished: JournalEffectSpan[],
  ): void {
    const previous = spans.get(effect.instanceId);

    const at = Math.max(0, (effect.atUs - segment.startedAtUs) / 1000);

    if (effect.removed === true) {
      if (previous !== undefined) {
        finished.push({ ...previous, endMs: Math.min(previous.endMs, at) });
        spans.delete(effect.instanceId);
      }

      return;
    }

    const end = this.end(effect, segment, at);

    if (
      previous !== undefined &&
      (previous.endMs < at === true || previous.key !== this.key(effect))
    ) {
      finished.push({ ...previous, endMs: Math.min(previous.endMs, at) });
      spans.delete(effect.instanceId);
    }

    const current = spans.get(effect.instanceId);

    spans.set(effect.instanceId, {
      key: this.key(effect),
      skillId: effect.skillId,
      effectId: effect.effectId,
      startMs: current?.startMs ?? at,
      endMs: end,
    });
  }

  private end(effect: JournalEffect, segment: JournalSegment, at: number): number {
    const duration = segment.spanMs ?? segment.elapsedMs;

    if (effect.expiresAtUs !== null && effect.expiresAtUs !== undefined) {
      return Math.min(duration, (effect.expiresAtUs - segment.startedAtUs) / 1000);
    }

    return effect.durationMs === null ? duration : Math.min(duration, at + effect.durationMs);
  }

  private key(effect: JournalEffect): string {
    return effect.effectId === null
      ? 'skill:' + String(effect.skillId)
      : 'effect:' + String(effect.effectId);
  }

  private add(groups: Map<string, JournalEffectSpan[]>, span: JournalEffectSpan): void {
    if (span.endMs <= span.startMs === true) {
      return;
    }

    const spans = groups.get(span.key) ?? [];

    spans.push(span);
    groups.set(span.key, spans);
  }

  private row(spans: readonly JournalEffectSpan[]): JournalUptimeRow {
    return this.union({
      key: spans[0]?.key ?? '',
      skillId: spans[0]?.skillId ?? null,
      effectId: spans[0]?.effectId ?? null,
      milliseconds: 0,
      intervals: spans.map((span) => [span.startMs, span.endMs]),
    });
  }

  private union(row: JournalUptimeRow): JournalUptimeRow {
    const intervals: [number, number][] = [];

    for (const span of [...row.intervals].sort((a, b) => a[0] - b[0])) {
      const last = intervals.at(-1);

      if (last !== undefined && last[1] >= span[0] === true) {
        last[1] = Math.max(last[1], span[1]);
      } else {
        intervals.push([span[0], span[1]]);
      }
    }

    return {
      ...row,
      intervals,
      milliseconds: intervals.reduce((sum, span) => sum + span[1] - span[0], 0),
    };
  }
}
