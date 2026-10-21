import { CharacterClass } from './character-class';

export class ClassPalette {
  public readonly defaults: Readonly<Record<CharacterClass, string>> = {
    [CharacterClass.Gladiator]: '#c28b64',
    [CharacterClass.Templar]: '#d2ba6c',
    [CharacterClass.Assassin]: '#b38cce',
    [CharacterClass.Ranger]: '#83b974',
    [CharacterClass.Sorcerer]: '#74b7db',
    [CharacterClass.Elementalist]: '#d88977',
    [CharacterClass.Cleric]: '#ddd6bb',
    [CharacterClass.Chanter]: '#abc875',
  };

  public color(name: CharacterClass | null | undefined, fallback: string): string {
    if (name === null || name === undefined || Object.hasOwn(this.defaults, name) === false) {
      return fallback;
    }

    return 'var(--class-' + name.toLowerCase() + ', ' + this.defaults[name] + ')';
  }

  public apply(root: HTMLElement, colors: Readonly<Partial<Record<CharacterClass, string>>>): void {
    for (const name of Object.values(CharacterClass)) {
      root.style.setProperty('--class-' + name.toLowerCase(), colors[name] ?? this.defaults[name]);
    }
  }
}
