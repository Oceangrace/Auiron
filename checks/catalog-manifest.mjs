import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import path from 'node:path';

export class CatalogManifest {
  constructor(root) {
    this.root = root;
  }

  read(name) {
    return JSON.parse(readFileSync(new URL(name, this.root), 'utf8'));
  }

  files(configuration) {
    const catalog = configuration.catalog;
    const names = [
      catalog.player,
      catalog.enemy,
      catalog.zones,
      catalog.npcs,
      catalog.runtime,
      catalog.skillNames,
    ];

    for (const locale of configuration.localization.bundled) {
      if (locale.catalog !== undefined && locale.catalog !== null) {
        const name = path.posix.relative(catalog.directory, locale.catalog);

        if (name.startsWith('../') === true || path.posix.isAbsolute(name) === true) {
          throw new Error('Bundled translation must be inside the catalog directory');
        }

        names.push(name);
      }
    }

    return [...new Set(names)].sort();
  }

  checksum(directory, name, manifest) {
    const bytes = readFileSync(new URL(directory + name, this.root));
    const data = JSON.parse(bytes.toString('utf8'));

    if (
      data.schemaVersion !== manifest.schemaVersion ||
      data.clientBuild !== manifest.clientBuild ||
      data.catalogVersion !== manifest.catalogVersion
    ) {
      throw new Error('Catalog header mismatch: ' + name);
    }

    return { sha256: createHash('sha256').update(bytes).digest('hex'), bytes: bytes.length };
  }

  run(write) {
    const configuration = this.read('data/application.json');
    const directory = 'data/' + configuration.catalog.directory + '/';
    const filename = directory + configuration.catalog.manifest;
    const manifest = this.read(filename);
    const names = this.files(configuration);
    const files = Object.fromEntries(
      names.map((name) => [name, this.checksum(directory, name, manifest)]),
    );

    if (write === true) {
      writeFileSync(
        new URL(filename, this.root),
        JSON.stringify({ ...manifest, files }, null, 2) + '\n',
      );
      console.log('Catalog manifest updated (' + names.length + ' data files).');

      return;
    }

    if (JSON.stringify(Object.keys(manifest.files).sort()) !== JSON.stringify(names)) {
      throw new Error('Catalog manifest file set is out of date. Run npm run catalog:manifest');
    }

    for (const [name, checksum] of Object.entries(files)) {
      const saved = manifest.files[name];

      if (saved.sha256 !== checksum.sha256 || saved.bytes !== checksum.bytes) {
        throw new Error('Catalog checksum mismatch: ' + name + '. Run npm run catalog:manifest');
      }
    }

    console.log('Catalog headers and checksums passed (' + names.length + ' data files).');
  }
}

new CatalogManifest(new URL('../', import.meta.url)).run(process.argv.includes('--write'));
