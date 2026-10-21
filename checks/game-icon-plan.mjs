import { readFileSync } from 'node:fs';

export class GameIconPlan {
  constructor(root) {
    this.root = root;
    this.configuration = this.read('data/application.json');
    this.audit = this.read('data/game-icon-audit.json');
  }

  read(file) {
    return JSON.parse(readFileSync(new URL(file, this.root), 'utf8'));
  }

  references() {
    const config = this.configuration.catalog;
    const directory = 'data/' + config.directory + '/';
    const player = this.read(directory + config.player);
    const enemies = this.read(directory + config.enemy);
    const runtime = this.read(directory + config.runtime);
    const entries = [
      ...player.skills,
      ...player.effects,
      ...enemies.entries.flatMap((entry) => [...entry.skills, ...entry.effects]),
      ...Object.values(runtime.skills),
      ...Object.values(runtime.effects),
    ];

    return new Set(
      entries
        .map((entry) => entry.icon)
        .filter((icon) => typeof icon === 'string' && icon.length !== 0),
    );
  }

  validate(path) {
    if (/^[A-Za-z0-9_]+(?:\/[A-Za-z0-9_]+)*$/.test(path) === false || path.length > 220 === true) {
      throw new Error('Invalid icon path: ' + path);
    }
  }

  source(path) {
    const visited = new Set();
    let source = path;

    while (this.audit.aliases[source] !== undefined) {
      if (visited.has(source) === true) {
        throw new Error('Icon alias cycle: ' + path);
      }

      visited.add(source);
      source = this.audit.aliases[source];
    }

    this.validate(source);
    return source.split('/').at(-1);
  }

  unavailable() {
    return this.audit.unavailable;
  }

  downloads() {
    const references = this.references();
    const downloads = new Map();
    const owners = new Map();

    if (this.audit.clientBuild !== this.configuration.catalog.clientBuild) {
      throw new Error('Icon manifest build mismatch');
    }

    for (const path of [...this.audit.unavailable, ...Object.keys(this.audit.aliases)]) {
      if (references.has(path) === false) {
        throw new Error('Unreferenced icon manifest entry: ' + path);
      }
    }

    for (const path of references) {
      this.validate(path);

      if (this.audit.unavailable.includes(path) === true) {
        continue;
      }

      const name = this.source(path);
      const file = path.replaceAll('/', '_').toLowerCase() + '.png';

      if (owners.has(file) === true && owners.get(file) !== name) {
        throw new Error('Conflicting icon filenames: ' + path);
      }

      owners.set(file, name);
      const files = downloads.get(name) ?? new Set();
      files.add(file);
      downloads.set(name, files);
    }

    return downloads;
  }
}
