<script setup lang="ts">
import BossRuleFields from './boss-rule-fields.vue';
import ValueRuleFields from './value-rule-fields.vue';
import type { AppearancePreview } from './appearance-preview';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import type { CatalogClient } from '../triggers/catalog-client';
import type { TriggerGroup } from '../model/rules/trigger-group';
import type { TriggerRule } from '../model/rules/trigger-rule';
import type { CatalogSelection } from '../model/catalog-selection';
import type { CatalogItem } from '../model/catalog-item';
import { LogicMode } from '../model/rules/logic-mode';
import { TriggerType } from '../model/rules/trigger-type';
import { GameEventType } from '../model/rules/game-event-type';
import { CompoundRuleFactory } from './compound-rule-factory';
import EffectRuleFields from './effect-rule-fields.vue';
import EventRuleFields from './event-rule-fields.vue';
import RuleCatalogFields from './rule-catalog-fields.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
const props = defineProps<{
  widget: WidgetNode;
  session: EditorSession;
  catalog: CatalogClient;
  preview: AppearancePreview;
}>();

const { t } = useI18n();

const factory = new CompoundRuleFactory();

const sources = computed(() => [
  { value: '', label: t('modules.interfaces.rules.noData') },
  ...(props.widget.rules?.rules ?? [])
    .filter((rule) => rule.kind !== TriggerType.Value)
    .map((rule, index) => ({
      value: rule.id,
      label: String(index + 1) + ' · ' + t('modules.interfaces.rules.types.' + rule.kind),
    })),
]);

const firstDataRule = (rules: readonly TriggerRule[]): string | null =>
  rules.find((rule) => rule.kind !== TriggerType.Value)?.id ?? null;

const groupFor = (
  rules: readonly TriggerRule[],
  previous: TriggerGroup | null,
): TriggerGroup | null => {
  if (rules.length === 0) {
    return null;
  }

  const first = firstDataRule(rules);

  const current = previous?.dataRuleId ?? null;

  return {
    mode: previous?.mode ?? LogicMode.All,
    dataRuleId: rules.some((rule) => rule.id === current) ? current : first,
    rules,
  };
};

const update = (rules: readonly TriggerRule[], clearSelection: string | null = null): void => {
  props.session.updateSelected((node) => ({
    ...node,
    ruleSelections: Object.fromEntries(
      Object.entries(node.ruleSelections).filter(
        ([id]) => id !== clearSelection && rules.some((rule) => rule.id === id),
      ),
    ),
    rules: groupFor(rules, node.rules),
  }));
};

const changedEvent = (previous: TriggerRule | undefined, value: TriggerRule): boolean =>
  previous?.kind === TriggerType.Event &&
  value.kind === TriggerType.Event &&
  previous.trigger.event !== value.trigger.event;

const clearEventIds = (rule: TriggerRule): TriggerRule =>
  rule.kind === TriggerType.Event
    ? { ...rule, trigger: { ...rule.trigger, skillId: null, effectId: null } }
    : rule;

const change = (index: number, value: TriggerRule): void => {
  const reset = changedEvent(props.widget.rules?.rules[index], value);

  const next = reset ? clearEventIds(value) : value;

  update(
    props.widget.rules?.rules.map((rule, at) => (at === index ? next : rule)) ?? [],
    reset ? value.id : null,
  );
};

const ids = (rule: TriggerRule): readonly number[] => {
  if (rule.kind === TriggerType.Effect) {
    return rule.trigger.effects;
  }

  if (rule.kind === TriggerType.Value || rule.kind === TriggerType.Boss) {
    return [];
  }

  return [rule.trigger.skillId, rule.trigger.effectId].filter((id): id is number => id !== null);
};

const choose = (index: number, selection: CatalogSelection, item: CatalogItem): void => {
  const rule = props.widget.rules?.rules[index];

  if (rule === undefined || rule.kind === TriggerType.Value || rule.kind === TriggerType.Boss) {
    return;
  }

  const next =
    rule.kind === TriggerType.Effect
      ? { ...rule, trigger: { ...rule.trigger, effects: [] } }
      : { ...rule, trigger: { ...rule.trigger, skillId: null, effectId: null } };

  change(index, next);
  props.session.updateSelected((node) => ({
    ...node,
    appearance: { ...node.appearance, iconPath: item.icon },
    ruleSelections: {
      ...node.ruleSelections,
      [rule.id]: [selection],
    },
  }));
};
</script>
<template>
  <section class="compound-editor">
    <h3>{{ t('modules.interfaces.rules.title') }}</h3>
    <div class="inspector-block">
      <UiSwitch
        :model-value="preview.enabled.value"
        :label="t('modules.interfaces.conditions.preview')"
        @update:model-value="preview.enable($event)"
      />
      <div class="choice-row">
        <UiButton
          v-for="kind in [
            TriggerType.Effect,
            TriggerType.Event,
            TriggerType.Boss,
            TriggerType.Value,
          ]"
          :key="kind"
          class="is-add"
          :disabled="(widget.rules?.rules.length ?? 0) >= 8"
          @click="update([...(widget.rules?.rules ?? []), factory.create(kind)])"
          >＋ {{ t('modules.interfaces.rules.types.' + kind) }}</UiButton
        >
      </div>
      <template v-if="widget.rules !== null">
        <div class="field-grid">
          <UiField :label="t('modules.interfaces.conditions.mode')">
            <UiSelect
              :model-value="widget.rules.mode"
              :options="
                Object.values(LogicMode).map((value) => ({
                  value,
                  label: t('modules.interfaces.conditions.modes.' + value),
                }))
              "
              :label="t('modules.interfaces.conditions.mode')"
              @update:model-value="
                session.updateSelected((node) => ({
                  ...node,
                  rules: node.rules === null ? null : { ...node.rules, mode: $event },
                }))
              "
            />
          </UiField>
          <UiField :label="t('modules.interfaces.rules.dataRule')">
            <UiSelect
              :model-value="widget.rules.dataRuleId ?? ''"
              :options="sources"
              :label="t('modules.interfaces.rules.dataRule')"
              @update:model-value="
                session.updateSelected((node) => ({
                  ...node,
                  rules:
                    node.rules === null
                      ? null
                      : { ...node.rules, dataRuleId: $event === '' ? null : $event },
                }))
              "
            />
          </UiField>
        </div>
        <section v-for="(rule, index) in widget.rules.rules" :key="rule.id" class="compound-card">
          <header>
            <strong>{{ index + 1 }}. {{ t('modules.interfaces.rules.types.' + rule.kind) }}</strong
            ><UiButton
              :danger="true"
              @click="update(widget.rules.rules.filter((_, at) => at !== index))"
              >{{ t('modules.interfaces.remove') }}</UiButton
            >
          </header>
          <UiSelect
            v-if="preview.enabled.value && rule.kind !== TriggerType.Value"
            :model-value="
              String(
                preview.context.value.fields?.['rule.' + rule.id + '.active'] === undefined
                  ? (preview.context.value.active ?? '?')
                  : (preview.context.value.fields['rule.' + rule.id + '.active'] ?? '?'),
              )
            "
            :options="[
              { value: '?', label: t('modules.interfaces.conditions.unknown') },
              { value: 'true', label: t('modules.interfaces.conditions.active') },
              { value: 'false', label: t('modules.interfaces.conditions.inactive') },
            ]"
            :label="t('modules.interfaces.conditions.preview')"
            @update:model-value="
              preview.update({
                fields: {
                  ...preview.context.value.fields,
                  ['rule.' + rule.id + '.active']: $event === '?' ? null : $event === 'true',
                },
              })
            "
          />
          <UiSwitch
            :model-value="rule.observe"
            :label="t('modules.interfaces.rules.observe')"
            @update:model-value="change(index, { ...rule, observe: $event })"
          />
          <EffectRuleFields
            v-if="rule.kind === TriggerType.Effect"
            :trigger="rule.trigger"
            @change="change(index, { ...rule, trigger: $event })"
          />
          <EventRuleFields
            v-else-if="rule.kind === TriggerType.Event"
            :catalog="catalog"
            :trigger="rule.trigger"
            @change="change(index, { ...rule, trigger: $event })"
          />
          <BossRuleFields
            v-else-if="rule.kind === TriggerType.Boss"
            :catalog="catalog"
            :trigger="rule.trigger"
            @change="change(index, { ...rule, trigger: $event })"
          />
          <ValueRuleFields
            v-else
            :trigger="rule.trigger"
            :formulas="session.document()?.formulas ?? []"
            :rules="widget.rules.rules"
            @change="change(index, { ...rule, trigger: $event })"
          />
          <template v-if="rule.kind === TriggerType.Effect || rule.kind === TriggerType.Event">
            <RuleCatalogFields
              :catalog="catalog"
              :ids="ids(rule)"
              :effect="
                rule.kind === TriggerType.Effect ||
                [
                  GameEventType.EffectApplied,
                  GameEventType.EffectUpdated,
                  GameEventType.EffectRemoved,
                ].includes(rule.trigger.event)
              "
              :selected="widget.ruleSelections[rule.id] ?? []"
              @choose="(selection, item) => choose(index, selection, item)"
            />
            <UiButton
              @click="
                session.updateSelected((node) => ({
                  ...node,
                  ruleSelections: { ...node.ruleSelections, [rule.id]: [] },
                }));
                change(
                  index,
                  rule.kind === TriggerType.Effect
                    ? { ...rule, trigger: { ...rule.trigger, effects: [] } }
                    : { ...rule, trigger: { ...rule.trigger, skillId: null, effectId: null } },
                );
              "
              >{{ t('modules.interfaces.rules.clearFilter') }}</UiButton
            >
          </template>
        </section>
      </template>
    </div>
  </section>
</template>
<style scoped>
.compound-card {
  display: grid;
  gap: 14px;
  min-width: 0;
  padding-top: 18px;
  border-top: 1px solid var(--prism-border);
}
.compound-editor > .inspector-block {
  gap: 20px;
}
.compound-card > .ui-switch {
  justify-self: start;
}
.compound-card > .field-grid {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}
.compound-card > .field-grid.value-rule-fields {
  grid-template-columns: minmax(200px, 2fr) minmax(100px, 1fr) minmax(130px, 1fr);
}
@container (max-width: 700px) {
  .compound-card > .field-grid.value-rule-fields {
    grid-template-columns: minmax(0, 1fr);
  }
}
@container (max-width: 540px) {
  .compound-card > .field-grid {
    grid-template-columns: minmax(0, 1fr);
  }
}
.compound-card > header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 10px;
  font-size: 12px;
}
.compound-card > header .ui-button {
  min-height: 24px;
  padding: 0 6px;
  border-color: transparent;
  background: transparent;
  font-size: 12px;
}
.compound-card > .ui-button {
  justify-self: start;
  border-color: transparent;
  background: transparent;
  font-size: 12px;
}
</style>
