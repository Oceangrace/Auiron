import { shallowRef } from 'vue';

export class IconAssets {
  private readonly available = shallowRef<ReadonlySet<string>>(new Set());

  public add(files: readonly string[]): void {
    this.available.value = new Set([...this.available.value, ...files]);
  }

  public labels(labels: Readonly<Record<string, string>>): Readonly<Record<string, string>> {
    return Object.fromEntries(
      Object.entries(labels).filter(([file]) => this.available.value.has(file)),
    );
  }

  public url(path: string | null): string | null {
    if (path === null) {
      return null;
    }

    const file = path.startsWith('game:')
      ? path.slice(5)
      : path.replaceAll('/', '_').toLowerCase() + '.png';

    if (/^[a-z0-9_]+\.png$/.test(file) === false || this.available.value.has(file) === false) {
      return null;
    }

    return 'http://game-icon.localhost/' + file;
  }
}
