import { shallowReadonly, shallowRef } from 'vue';
import type { ShallowRef } from 'vue';
import type { TriggerDraft } from './trigger-draft';
import type { TriggerDefinition } from '../model/trigger-definition';
import type { CatalogItem } from '../model/catalog-item';
import type { CatalogForm } from '../model/catalog-form';
import type { CatalogSelection } from '../model/catalog-selection';
import { CatalogScope } from '../model/catalog-scope';
import { ObservationKind } from '../model/observation-kind';
import { TriggerSubject } from '../model/trigger-subject';

export class TriggerController {
  private readonly state = shallowRef<TriggerDraft>({
    subject: TriggerSubject.SelfPlayer,
    observation: ObservationKind.EffectPresent,
    actorKey: null,
    contextKey: null,
    scope: CatalogScope.Effects,
    selection: null,
    seconds: 3,
    iconPath: null,
  });

  public getState(): Readonly<ShallowRef<TriggerDraft>> {
    return shallowReadonly(this.state);
  }

  public restore(trigger: TriggerDefinition | null, iconPath: string | null): void {
    if (trigger === null) {
      this.state.value = {
        subject: TriggerSubject.SelfPlayer,
        observation: ObservationKind.EffectPresent,
        actorKey: null,
        contextKey: null,
        scope: CatalogScope.Effects,
        selection: null,
        seconds: 3,
        iconPath,
      };

      return;
    }

    this.state.value = {
      subject: trigger.subject,
      observation: trigger.observation,
      actorKey: trigger.actorKey,
      contextKey: trigger.selection.contextKey,
      scope: trigger.selection.scope,
      selection: trigger.selection,
      seconds: trigger.displaySeconds ?? 3,
      iconPath,
    };
  }

  public setSubject(subject: TriggerSubject): void {
    const enemy = subject === TriggerSubject.Enemy;

    this.state.value = {
      ...this.state.value,
      subject,
      selection: null,
      actorKey: null,
      contextKey: null,
      observation: enemy ? ObservationKind.SkillUsed : ObservationKind.EffectPresent,
      scope: enemy ? CatalogScope.EnemySkills : CatalogScope.Effects,
    };
  }

  public setObservation(observation: ObservationKind): void {
    const scope =
      observation === ObservationKind.EffectPresent ? CatalogScope.Effects : CatalogScope.Skills;

    this.state.value = {
      ...this.state.value,
      observation,
      scope,
      selection: scope === this.state.value.scope ? this.state.value.selection : null,
      contextKey: null,
    };
  }

  public setEnemy(item: CatalogItem): void {
    const actorKey = this.state.value.subject === TriggerSubject.Enemy ? item.key : null;

    this.state.value = { ...this.state.value, actorKey, contextKey: item.key, selection: null };
  }

  public setEffectScope(scope: CatalogScope): void {
    this.state.value = { ...this.state.value, scope, contextKey: null, selection: null };
  }

  public setSeconds(seconds: number): void {
    this.state.value = { ...this.state.value, seconds };
  }

  public choose(item: CatalogItem, form: CatalogForm, version: string): void {
    const selection: CatalogSelection = {
      catalogVersion: version,
      scope: this.state.value.scope,
      contextKey: this.state.value.contextKey,
      entryKey: item.key,
      choiceKey: form.key,
    };

    this.state.value = { ...this.state.value, selection, iconPath: item.icon };
  }

  public definition(): TriggerDefinition | null {
    const draft = this.state.value;

    if (draft.selection === null) {
      return null;
    }

    return {
      subject: draft.subject,
      observation: draft.observation,
      actorKey: draft.actorKey,
      selection: draft.selection,
      displaySeconds: draft.observation === ObservationKind.SkillUsed ? draft.seconds : null,
    };
  }
}
