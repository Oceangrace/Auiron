import { LoadState } from './load-state';

export interface LoadRules {
  readonly mapIds: readonly number[];
  readonly dungeonIds: readonly number[];
  readonly npcIds: readonly number[];
  readonly inCombat: LoadState;
  readonly inGroup: LoadState;
  readonly playerClass: string;
  readonly zone: string;
}

export const defaultLoadRules: LoadRules = {
  mapIds: [],
  dungeonIds: [],
  npcIds: [],
  inCombat: LoadState.Any,
  inGroup: LoadState.Any,
  playerClass: '',
  zone: '',
};
