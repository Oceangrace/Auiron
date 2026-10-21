export class GameIdList {
  public parse(text: string, multiple: boolean, maximum = 32): readonly number[] | null {
    if (text.trim() === '') {
      return [];
    }

    const parts = text.trim().split(/[\s,;]+/u);

    const limit = multiple ? maximum : 1;

    if (
      parts.length > limit === true ||
      parts.some((part) => /^[1-9]\d{0,9}$/u.test(part) === false) === true
    ) {
      return null;
    }

    const ids = parts.map(Number);

    return ids.every((id) => id <= 4294967295) ? [...new Set(ids)] : null;
  }
}
