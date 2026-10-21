export interface ReleaseInfo {
  readonly currentVersion: string;
  readonly latestVersion: string | null;
  readonly available: boolean;
}
