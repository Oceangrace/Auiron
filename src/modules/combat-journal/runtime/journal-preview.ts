import { CombatCounter } from '../model/combat-counter';
import { JournalClass } from '../model/journal-class';
import { JournalPreviewSkill } from '../model/journal-preview-skill';
import type { JournalRecord } from '../model/journal-record';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalUnit } from '../model/journal-unit';

export class JournalPreview {
  private readonly names = new Map<number, string>([
    [JournalPreviewSkill.Strike, 'strike'],
    [JournalPreviewSkill.Combo, 'combo'],
    [JournalPreviewSkill.Area, 'area'],
    [JournalPreviewSkill.Healing, 'healing'],
    [JournalPreviewSkill.EnemyStrike, 'enemyStrike'],
  ]);

  public skill(id: number | null, text: (key: string) => string): string {
    const key = id === null ? undefined : this.names.get(id);

    return key === undefined ? '—' : text(key);
  }

  public create(text: (key: string) => string, elapsedMs = 60_000): JournalSegment {
    const elapsed = Math.max(0, elapsedMs);

    const seconds = Math.max(0.1, Math.min(elapsed % 45_000, 40_000) / 1000);

    const units = this.units(text);

    const records = units
      .filter((unit) => unit.player)
      .flatMap((unit) => this.actions(unit.id, seconds));

    records.push(
      this.damage(8, 6, JournalPreviewSkill.Strike, this.amount(1, seconds) * 0.05, seconds),
    );

    return {
      id: 'journal-preview-' + String(Math.floor(elapsed / 45_000)),
      startedAtUs: 0,
      endedAtUs: seconds * 1_000_000,
      elapsedMs: seconds * 1000,
      mapId: null,
      npcId: null,
      playerId: 1,
      units: Object.fromEntries(units.map((unit) => [unit.id, unit])),
      records,
    };
  }

  private units(text: (key: string) => string): readonly JournalUnit[] {
    const classes = [
      JournalClass.Ranger,
      JournalClass.Gladiator,
      JournalClass.Templar,
      JournalClass.Cleric,
      JournalClass.Sorcerer,
    ];

    const players = classes.map((className, index) => ({
      id: index + 1,
      ownerId: null,
      name: text('player' + String(index + 1)),
      className,
      npcId: null,
      player: true,
      group: true,
    }));

    return [
      ...players,
      this.other(6, text('target1'), null),
      this.other(7, text('target2'), null),
      this.other(8, text('summon'), 1),
    ];
  }

  private other(id: number, name: string, ownerId: number | null): JournalUnit {
    return { id, name, ownerId, className: null, npcId: null, player: false, group: false };
  }

  private amount(id: number, seconds: number): number {
    const phase = ((id - 1) * Math.PI * 2) / 5;

    return Math.round(
      (9500 + id * 120) * seconds + 8500 * (Math.cos(phase) - Math.cos(phase + seconds)),
    );
  }

  private actions(id: number, seconds: number): JournalRecord[] {
    const amount = this.amount(id, seconds);

    return [
      this.damage(id, 6, JournalPreviewSkill.Strike, amount * 0.63, seconds),
      this.damage(id, 6, JournalPreviewSkill.Combo, amount * 0.27, seconds),
      this.damage(id, 7, JournalPreviewSkill.Area, amount * 0.1, seconds),
      this.damage(6, id, JournalPreviewSkill.EnemyStrike, amount * 0.1, seconds),
      this.record(
        id,
        id === 5 ? 1 : id + 1,
        JournalPreviewSkill.Healing,
        {
          [CombatCounter.Healing]: Math.round(amount * 0.2),
          [CombatCounter.HealHits]: Math.floor(seconds / 2) + 1,
        },
        seconds,
      ),
      this.record(
        id,
        id,
        null,
        { [CombatCounter.Deaths]: Math.floor(seconds / (8 + id * 3)) },
        seconds,
      ),
    ];
  }

  private damage(
    sourceId: number,
    recipientId: number,
    skill: JournalPreviewSkill,
    amount: number,
    seconds: number,
  ): JournalRecord {
    const damage = Math.round(amount);

    const hits = Math.floor(seconds * 2) + 1;

    return this.record(
      sourceId,
      recipientId,
      skill,
      {
        [CombatCounter.Damage]: damage,
        [CombatCounter.DirectDamage]: damage,
        [CombatCounter.Hits]: hits,
        [CombatCounter.DirectHits]: hits,
        [CombatCounter.CriticalHits]: Math.floor(hits * 0.3),
        [CombatCounter.BackHits]: Math.floor(hits * 0.2),
        [CombatCounter.Misses]: Math.floor(seconds / 4),
        [CombatCounter.Resists]: Math.floor(seconds / 7),
      },
      seconds,
    );
  }

  private record(
    sourceId: number,
    recipientId: number,
    skillId: JournalPreviewSkill | null,
    counters: Readonly<Partial<Record<CombatCounter, number>>>,
    seconds: number,
  ): JournalRecord {
    return {
      sourceId,
      recipientId,
      skillId,
      statistics: {
        counters,
        firstAtUs: 0,
        lastAtUs: seconds * 1_000_000,
        minimum: null,
        maximum: null,
        complete: true,
      },
    };
  }
}
