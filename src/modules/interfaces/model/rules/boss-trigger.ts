import type { BossEvent } from './boss-event';

export interface BossTrigger {
  readonly bossId: number;
  readonly event: BossEvent;
  readonly beforeSeconds: number;
  readonly durationMs: number;
}
