import { readdir, readFile } from 'node:fs/promises';
import { dirname, extname, relative, resolve } from 'node:path';
import ts from 'typescript';
import { parse } from '@vue/compiler-sfc';

class ArchitectureGraph {
  edges = new Map();
  files = new Set();
  nativeAliases = new Map();

  async check() {
    const frontend = await this.collect('src', ['.ts', '.vue']);
    const native = await this.collect('src-tauri/src', ['.rs']);

    this.files = new Set([...frontend, ...native]);

    for (const file of this.files) {
      this.edges.set(file, new Set());
    }

    for (const file of frontend) {
      await this.frontend(file);
    }

    for (const file of native) {
      await this.nativeExports(file);
    }

    for (const file of native) {
      await this.native(file);
    }

    this.checkCycles();
    process.stdout.write(
      `Architecture boundaries and import cycles passed (${frontend.length} TS/Vue, ${native.length} Rust files).\n`,
    );
  }

  async collect(directory, extensions) {
    const entries = await readdir(directory, { withFileTypes: true });
    const files = [];

    for (const entry of entries) {
      const path = resolve(directory, entry.name);

      if (entry.isDirectory() === true) {
        files.push(...(await this.collect(path, extensions)));
      } else if (extensions.includes(extname(path)) === true) {
        files.push(path);
      }
    }

    return files;
  }

  name(file) {
    return relative(process.cwd(), file).replaceAll('\\', '/');
  }

  async frontend(file) {
    let source = await readFile(file, 'utf8');

    if (extname(file) === '.vue') {
      const parsed = parse(source, { filename: file });

      if (parsed.errors.length !== 0) {
        throw new Error(`${this.name(file)}: Vue parse failed`);
      }

      source = [parsed.descriptor.script?.content, parsed.descriptor.scriptSetup?.content]
        .filter(Boolean)
        .join('\n');
    }

    const syntax = ts.createSourceFile(
      file,
      source,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TS,
    );

    this.visit(syntax, file);
  }

  visit(node, file) {
    if (
      ts.isImportDeclaration(node) === true &&
      ts.isStringLiteral(node.moduleSpecifier) === true
    ) {
      this.frontendImport(file, node.moduleSpecifier.text);
    }

    if (ts.isExportDeclaration(node) === true && node.moduleSpecifier !== undefined) {
      if (ts.isStringLiteral(node.moduleSpecifier) === true) {
        this.frontendImport(file, node.moduleSpecifier.text);
      }
    }

    if (
      ts.isCallExpression(node) === true &&
      node.expression.kind === ts.SyntaxKind.ImportKeyword
    ) {
      const argument = node.arguments[0];

      if (argument !== undefined && ts.isStringLiteral(argument) === true) {
        this.frontendImport(file, argument.text);
      } else {
        throw new Error(`${this.name(file)}: dynamic module discovery is prohibited`);
      }
    }

    ts.forEachChild(node, (child) => this.visit(child, file));
  }

  frontendImport(file, specifier) {
    const name = this.name(file);

    if (name.startsWith('src/core/') === true && /^(vue|@tauri-apps\/)/.test(specifier) === true) {
      throw new Error(`${name}: core must not depend on Vue or Tauri`);
    }

    if (
      specifier.startsWith('@tauri-apps/') === true &&
      name.startsWith('src/integrations/tauri/') === false
    ) {
      throw new Error(`${name}: use the explicit desktop contract`);
    }

    if (specifier.startsWith('.') === false) {
      return;
    }

    const stem = resolve(dirname(file), specifier);
    const target = [stem, stem + '.ts', stem + '.vue', resolve(stem, 'index.ts')].find(
      (candidate) => this.files.has(candidate),
    );

    if (target === undefined) {
      if (/\.(css|json|woff2)$/.test(specifier) === false) {
        throw new Error(`${name}: unresolved project import ${specifier}`);
      }

      return;
    }

    this.frontendBoundary(name, this.name(target));
    this.edges.get(file).add(target);
  }

  frontendBoundary(source, target) {
    if (
      source.startsWith('src/core/') === true &&
      /^src\/(config|renderer|modules|integrations)\//.test(target) === true
    ) {
      throw new Error(`${source}: core imports implementation ${target}`);
    }

    if (
      source.startsWith('src/integrations/') === true &&
      /^src\/(config|renderer|modules)\//.test(target) === true
    ) {
      throw new Error(`${source}: integration imports feature/UI ${target}`);
    }

    const sourceModule = /^src\/modules\/([^/]+)\//.exec(source)?.[1];
    const targetModule = /^src\/modules\/([^/]+)\//.exec(target)?.[1];

    if (sourceModule !== undefined && targetModule !== undefined && sourceModule !== targetModule) {
      throw new Error(`${source}: direct cross-module import ${target}`);
    }

    if (sourceModule !== undefined && target.startsWith('src/config/') === true) {
      throw new Error(`${source}: feature imports composition root`);
    }

    const root = ['src/renderer/main.ts', 'src/renderer/app.vue'].includes(source);

    if (
      root === false &&
      source.startsWith('src/renderer/') === true &&
      /^src\/(modules|integrations|config)\//.test(target) === true
    ) {
      throw new Error(`${source}: shared shell must not own feature implementations`);
    }
  }

  async native(file) {
    const source = await readFile(file, 'utf8');
    const name = this.name(file).replace('src-tauri/src/', '');

    for (const match of source.matchAll(/\buse\s+([^;]+);/g)) {
      const imported = match[1].replaceAll(/\s+/g, '');
      const parts = this.nativeParts(name, imported);

      if (parts === null) {
        if (name.startsWith('core/') === true && /^tauri(?:::|$)/.test(imported) === true) {
          throw new Error(`${name}: core must not import Tauri`);
        }

        continue;
      }

      const resolved = this.nativeResolve(parts);

      this.nativeBoundary(name, resolved.join('/'));
      const target = this.nativeTarget(resolved);

      if (target !== null && target !== file) {
        this.edges.get(file).add(target);
      }
    }
  }

  async nativeExports(file) {
    const source = await readFile(file, 'utf8');
    const name = this.name(file).replace('src-tauri/src/', '');
    const module = this.nativeModule(name);

    const imports = name.endsWith('/mod.rs')
      ? /\b(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);/g
      : /\bpub(?:\([^)]*\))?\s+use\s+([^;]+);/g;

    for (const match of source.matchAll(imports)) {
      const parts = this.nativeParts(name, match[1].replaceAll(/\s+/g, ''));

      if (parts !== null) {
        this.nativeAliases.set([...module, parts.at(-1)].join('::'), parts);
      }
    }
  }

  nativeModule(name) {
    const parts = name.replace(/\.rs$/, '').split('/');

    if (['mod', 'lib', 'main'].includes(parts.at(-1)) === true) {
      parts.pop();
    }

    return parts;
  }

  nativeResolve(parts) {
    const seen = new Set();

    while (true) {
      const key = parts.join('::');

      if (seen.has(key) === true || seen.size >= 64 === true) {
        throw new Error(`Rust re-export cycle: ${key}`);
      }

      seen.add(key);
      const resolved = this.nativeAlias(parts);

      if (resolved === null) {
        return parts;
      }

      parts = resolved;
    }
  }

  nativeAlias(parts) {
    for (let length = parts.length; length !== 0; length--) {
      const alias = this.nativeAliases.get(parts.slice(0, length).join('::'));

      if (alias !== undefined) {
        return [...alias, ...parts.slice(length)];
      }
    }

    return null;
  }

  nativeParts(name, imported) {
    const parts = imported.split('::');
    let base = this.nativeModule(name);

    if (parts[0] === 'crate') {
      base = [];
      parts.shift();
    } else if (parts[0] === 'self') {
      parts.shift();
    } else if (parts[0] === 'super') {
      while (parts[0] === 'super') {
        parts.shift();
        base.pop();
      }
    } else if (
      ['application', 'core', 'modules', 'integrations', 'runtime'].includes(parts[0]) === true
    ) {
      base = [];
    } else {
      return null;
    }

    if (parts.some((part) => part.includes('{')) === true) {
      throw new Error(`${name}: use explicit paths for grouped internal imports`);
    }

    return [...base, ...parts];
  }

  nativeTarget(parts) {
    for (let length = parts.length; length !== 0; length--) {
      const path = parts.slice(0, length).join('/');
      const targets = [
        resolve('src-tauri/src', path + '.rs'),
        resolve('src-tauri/src', path, 'mod.rs'),
      ];
      const target = targets.find((candidate) => this.files.has(candidate));

      if (target !== undefined) {
        return target;
      }
    }

    return null;
  }

  nativeBoundary(source, target) {
    if (
      source.startsWith('core/') === true &&
      /^(application|runtime|modules|integrations)(\/|$)/.test(target) === true
    ) {
      throw new Error(`${source}: core imports implementation ${target}`);
    }

    if (
      source.startsWith('runtime/') === true &&
      /^(application|modules|integrations)(\/|$)/.test(target) === true
    ) {
      throw new Error(`${source}: runtime imports feature ${target}`);
    }

    if (
      source.startsWith('integrations/') === true &&
      /^(application|modules|runtime)(\/|$)/.test(target) === true
    ) {
      throw new Error(`${source}: integration imports application/UI ${target}`);
    }

    const sourceModule = /^modules\/([^/.]+)\//.exec(source)?.[1];
    const targetModule = /^modules\/([^/.]+)\//.exec(target)?.[1];

    if (sourceModule !== undefined && targetModule !== undefined && sourceModule !== targetModule) {
      throw new Error(`${source}: direct cross-module import ${target}`);
    }
  }

  checkCycles() {
    const visited = new Set();
    const active = new Set();

    for (const file of this.files) {
      this.cycle(file, visited, active, []);
    }
  }

  cycle(file, visited, active, chain) {
    if (active.has(file) === true) {
      throw new Error(
        'Import cycle: ' + [...chain, file].map((path) => this.name(path)).join(' → '),
      );
    }

    if (visited.has(file) === true) {
      return;
    }

    active.add(file);

    for (const target of this.edges.get(file)) {
      this.cycle(target, visited, active, [...chain, file]);
    }

    active.delete(file);
    visited.add(file);
  }
}

await new ArchitectureGraph().check();
