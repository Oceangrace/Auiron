import type { InterfaceDocument } from '../model/interface-document';
import type { InterfaceLocalization } from '../model/interface-localization';
import type { WidgetNode } from '../model/widget-node';
import type { ConditionChange } from '../model/condition-change';
import { ConditionProperty } from '../model/condition-property';

export class HudLocalization {
  public text(
    source: string,
    localization: InterfaceLocalization | undefined,
    locale: string,
  ): string {
    let code = locale.toLowerCase().replaceAll('_', '-');

    while (code !== '') {
      const value = this.translation(localization, code, source);

      if (value !== undefined && value !== '') {
        return value;
      }

      const end = code.lastIndexOf('-');

      code = end === -1 ? '' : code.slice(0, end);
    }

    return source;
  }

  public document(document: InterfaceDocument, locale: string): InterfaceDocument {
    if (
      document.localization === undefined ||
      Object.keys(document.localization.translations).length === 0
    ) {
      return document;
    }

    return {
      ...document,
      name: this.text(document.name, document.localization, locale),
      description: this.text(document.description, document.localization, locale),
      roots: document.roots.map((node) => this.node(node, document.localization, locale)),
    };
  }

  public sources(document: InterfaceDocument): readonly string[] {
    const values = new Set([document.name, document.description]);

    this.collect(document.roots, values);

    return [...values].filter((value) => value.trim() !== '');
  }

  public compatible(source: string, translated: string): boolean {
    return translated === '' || this.tokens(source) === this.tokens(translated);
  }

  private tokens(text: string): string {
    return JSON.stringify((text.match(/\{[^}]*\}/gu) ?? []).sort());
  }

  private translation(
    localization: InterfaceLocalization | undefined,
    code: string,
    source: string,
  ): string | undefined {
    const languages = localization?.translations;

    if (languages === undefined || Object.hasOwn(languages, code) === false) {
      return undefined;
    }

    const texts = languages[code];

    return texts !== undefined && Object.hasOwn(texts, source) ? texts[source] : undefined;
  }

  private condition(change: ConditionChange): string {
    if (
      change.property === ConditionProperty.Text ||
      change.property === ConditionProperty.IconLabel
    ) {
      return change.value;
    }

    return change.property === ConditionProperty.Sound ? (change.speechText ?? '') : '';
  }

  private collect(nodes: readonly WidgetNode[], values: Set<string>): void {
    for (const node of nodes) {
      const texts = [
        node.name,
        node.appearance.text,
        node.style.rightText,
        node.sounds.showText,
        node.sounds.hideText,
        ...node.headerButtons.map((button) => button.label),
        ...node.conditions.flatMap((condition) =>
          condition.changes.map((change) => this.condition(change)),
        ),
      ];

      if (node.group.headerEnabled === true) {
        values.add(node.group.headerText !== '' ? node.group.headerText : node.name);
      }

      for (const text of texts) {
        values.add(text);
      }

      this.collect(node.children, values);
    }
  }

  private node(
    node: WidgetNode,
    localization: InterfaceLocalization | undefined,
    locale: string,
  ): WidgetNode {
    const text = (value: string): string => this.text(value, localization, locale);

    return {
      ...node,
      name: text(node.name),
      group: { ...node.group, headerText: text(node.group.headerText) },
      appearance: { ...node.appearance, text: text(node.appearance.text) },
      style: { ...node.style, rightText: text(node.style.rightText) },
      sounds: {
        ...node.sounds,
        showText: text(node.sounds.showText),
        hideText: text(node.sounds.hideText),
      },
      headerButtons: node.headerButtons.map((button) => ({ ...button, label: text(button.label) })),
      conditions: node.conditions.map((condition) => ({
        ...condition,
        changes: condition.changes.map((change) => {
          if (
            change.property === ConditionProperty.Text ||
            change.property === ConditionProperty.IconLabel
          ) {
            return { ...change, value: text(change.value) };
          }

          if (change.property === ConditionProperty.Sound) {
            return { ...change, speechText: text(change.speechText ?? '') };
          }

          return change;
        }),
      })),
      children: node.children.map((child) => this.node(child, localization, locale)),
    };
  }
}
