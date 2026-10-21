import type { ReleaseInfo } from '../../core/updates/release-info';

export interface ReleaseState {
  readonly currentVersion: string | null;
  readonly info: ReleaseInfo | null;
  readonly checking: boolean;
  readonly checkFailed: boolean;
  readonly openFailed: boolean;
  readonly dismissedVersion: string | null;
}
