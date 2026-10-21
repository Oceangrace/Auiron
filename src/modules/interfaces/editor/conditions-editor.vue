<script setup lang="ts">
import UiMessage from '../../../renderer/components/ui/ui-message.vue';
import FieldPicker from './field-picker.vue';
import ConditionEffectFields from './condition-effect-fields.vue';
import PreviewFieldsEditor from './preview-fields-editor.vue';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { WidgetCondition } from '../model/widget-condition';
import type { ConditionCheck } from '../model/condition-check';
import type { ConditionChange } from '../model/condition-change';
import { ConditionChanges } from '../model/condition-changes';
import type { EditorSession } from './editor-session';
import type { AppearancePreview } from './appearance-preview';
import { ConditionField } from '../model/condition-field';
import { ConditionMode } from '../model/condition-mode';
import { ComparisonOperator } from '../model/comparison-operator';
import { DataKind } from '../model/rules/data-kind';
import { FieldCatalog } from './field-catalog';
import { ConditionProperty } from '../model/condition-property';
import { WidgetKind } from '../model/widget-kind';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import DataValueField from './data-value-field.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
const props = defineProps<{
  widget: WidgetNode;
  session: EditorSession;
  preview: AppearancePreview;
}>();

const conditionChanges = new ConditionChanges();

const fieldCatalog = new FieldCatalog();

const formulas = computed(() => props.session.document()?.formulas ?? []);

const triggerRules = computed(() => props.widget.rules?.rules ?? []);

const { t } = useI18n();

const context = props.preview.context;

const previewEnabled = props.preview.enabled;

const error = props.preview.error;

const fields = computed(() =>
  Object.values(ConditionField).map((value) => ({
    value,
    label: t('modules.interfaces.conditions.fields.' + value),
  })),
);

const operators = computed(() =>
  Object.values(ComparisonOperator).map((value) => ({
    value,
    label: t('modules.interfaces.conditions.operators.' + value),
  })),
);

const booleanOptions = computed(() => [
  { value: 'true', label: t('modules.interfaces.conditions.yes') },
  { value: 'false', label: t('modules.interfaces.conditions.no') },
]);

const booleanField = (test: ConditionCheck): boolean =>
  test.field === ConditionField.Active ||
  (test.field === ConditionField.Custom &&
    fieldCatalog.kind(test.path ?? '', formulas.value, triggerRules.value) === DataKind.Boolean);

const comparisons = (test: ConditionCheck): typeof operators.value =>
  booleanField(test)
    ? operators.value.filter((option) =>
        [ComparisonOperator.Equal, ComparisonOperator.NotEqual, test.operator].includes(
          option.value,
        ),
      )
    : operators.value;

const modes = computed(() =>
  Object.values(ConditionMode).map((value) => ({
    value,
    label: t('modules.interfaces.conditions.modes.' + value),
  })),
);

const properties = computed(() =>
  Object.values(ConditionProperty)
    .filter((value) => {
      if (value === ConditionProperty.Desaturate) {
        return props.widget.kind === WidgetKind.Icon;
      }

      return (
        props.session.tree.isGroup(props.widget) === false ||
        ![ConditionProperty.Text, ConditionProperty.Color, ConditionProperty.FontSize].includes(
          value,
        )
      );
    })
    .map((value) => ({
      value,
      label: t('modules.interfaces.conditions.properties.' + value.replaceAll('.', '_')),
    })),
);

const states = computed(() => [
  { value: '', label: t('modules.interfaces.conditions.unknown') },
  { value: 'true', label: t('modules.interfaces.conditions.active') },
  { value: 'false', label: t('modules.interfaces.conditions.inactive') },
]);

const values = ['remaining', 'stacks', 'progress'] as const;

const update = (rules: readonly WidgetCondition[]): void => {
  props.session.updateSelected((node) => ({ ...node, conditions: rules }));
};

const rule = (index: number, patch: Partial<WidgetCondition>): void => {
  update(
    props.widget.conditions.map((value, at) => (at === index ? { ...value, ...patch } : value)),
  );
};

const check = (index: number, at: number, patch: Partial<ConditionCheck>): void => {
  const item = props.widget.conditions[index];

  if (item !== undefined) {
    rule(index, {
      checks: item.checks.map((value, position) =>
        position === at ? { ...value, ...patch } : value,
      ),
    });
  }
};

const selectPath = (index: number, at: number, path: string): void => {
  const test = props.widget.conditions[index]?.checks[at];

  if (test === undefined) {
    return;
  }

  const boolean = fieldCatalog.kind(path, formulas.value, triggerRules.value) === DataKind.Boolean;

  const textValue = test.textValue ?? '0';

  check(index, at, {
    path,
    textValue: boolean ? String(textValue !== 'false') : textValue,
    operator:
      boolean && ![ComparisonOperator.Equal, ComparisonOperator.NotEqual].includes(test.operator)
        ? ComparisonOperator.Equal
        : test.operator,
  });
};

const change = (index: number, at: number, replacement: ConditionChange): void => {
  const item = props.widget.conditions[index];

  if (item !== undefined) {
    rule(index, {
      changes: item.changes.map((value, position) => (position === at ? replacement : value)),
    });
  }
};

const add = (): void => {
  update([
    ...props.widget.conditions,
    {
      id: crypto.randomUUID(),
      enabled: true,
      mode: ConditionMode.All,
      checks: [{ field: ConditionField.Active, operator: ComparisonOperator.Equal, value: 1 }],
      changes: [conditionChanges.create(ConditionProperty.Glow)],
    },
  ]);
};
</script>
<template>
  <section class="inspector-section conditions-editor">
    <div class="conditions-tools">
      <UiButton
        class="is-add"
        :disabled="widget.conditions.length >= 32 || session.isReadOnly()"
        @click="add"
        >＋ {{ t('modules.interfaces.conditions.add') }}</UiButton
      ><UiSwitch
        :model-value="previewEnabled"
        :label="t('modules.interfaces.conditions.preview')"
        @update:model-value="preview.enable($event)"
      />
    </div>
    <div v-if="previewEnabled" class="condition-preview">
      <UiField :label="t('modules.interfaces.conditions.fields.active')"
        ><UiSelect
          :model-value="context.active === null ? '' : String(context.active)"
          :options="states"
          :label="t('modules.interfaces.conditions.fields.active')"
          @update:model-value="
            preview.update({ active: $event === '' ? null : $event === 'true' })
          "
      /></UiField>
      <div v-for="field in values" :key="field">
        <UiSwitch
          :model-value="context[field] !== null"
          :label="t('modules.interfaces.conditions.fields.' + field)"
          @update:model-value="preview.update({ [field]: $event ? 1 : null })"
        />
        <UiNumber
          v-if="context[field] !== null"
          :model-value="context[field] ?? 0"
          :label="t('modules.interfaces.conditions.fields.' + field)"
          :min="0"
          :max="field === 'progress' ? 100 : field === 'stacks' ? 999 : 3600"
          @update:model-value="preview.update({ [field]: $event })"
        />
      </div>
    </div>
    <PreviewFieldsEditor v-if="previewEnabled" :preview="preview" :widget-id="widget.id" />
    <p v-if="error !== null" class="error-note" role="alert"><UiMessage :text="error" /></p>
    <section
      v-for="(item, index) in widget.conditions"
      :key="item.id"
      class="condition-card inspector-card"
    >
      <header>
        <UiSwitch
          :model-value="item.enabled"
          :label="t('modules.interfaces.conditions.rule', { n: index + 1 })"
          @update:model-value="rule(index, { enabled: $event })"
        /><UiButton
          class="condition-remove-rule"
          :danger="true"
          @click="update(widget.conditions.filter((_, at) => at !== index))"
          >{{ t('modules.interfaces.remove') }}</UiButton
        >
      </header>
      <div class="condition-group">
        <div class="condition-intro">
          <h3>{{ t('modules.interfaces.redesign.if') }}</h3>
          <p>{{ t('modules.interfaces.redesign.hints.checks') }}</p>
        </div>
        <div class="condition-settings">
          <UiField class="condition-mode" :label="t('modules.interfaces.conditions.mode')">
            <UiSelect
              :model-value="item.mode"
              :options="modes"
              :label="t('modules.interfaces.conditions.mode')"
              @update:model-value="rule(index, { mode: $event })"
            />
          </UiField>
          <div v-for="(test, at) in item.checks" :key="at" class="condition-check">
            <div class="condition-check-fields">
              <UiField :label="t('modules.interfaces.conditions.field')">
                <UiSelect
                  :model-value="test.field"
                  :options="fields"
                  :label="t('modules.interfaces.conditions.field')"
                  @update:model-value="
                    check(index, at, {
                      field: $event,
                      value: $event === ConditionField.Active ? 1 : 0,
                      path: 'player.hp.value',
                      textValue: '0',
                      operator:
                        $event === ConditionField.Active ? ComparisonOperator.Equal : test.operator,
                    })
                  "
                />
              </UiField>
              <UiField :label="t('modules.interfaces.conditions.operator')">
                <UiSelect
                  :model-value="test.operator"
                  :options="comparisons(test)"
                  :label="t('modules.interfaces.conditions.operator')"
                  @update:model-value="check(index, at, { operator: $event })"
                />
              </UiField>
              <UiField :label="t('modules.interfaces.conditions.value')">
                <UiSelect
                  v-if="booleanField(test)"
                  :model-value="
                    test.field === ConditionField.Active
                      ? String(test.value === 1)
                      : (test.textValue ?? '')
                  "
                  :options="booleanOptions"
                  :label="t('modules.interfaces.conditions.value')"
                  @update:model-value="
                    check(
                      index,
                      at,
                      test.field === ConditionField.Active
                        ? { value: $event === 'true' ? 1 : 0 }
                        : { textValue: $event },
                    )
                  "
                />
                <DataValueField
                  v-else-if="test.field === ConditionField.Custom"
                  :path="test.path ?? ''"
                  :formulas="formulas"
                  :rules="triggerRules"
                  :model-value="test.textValue ?? ''"
                  @update:model-value="check(index, at, { textValue: $event })"
                />
                <UiNumber
                  v-else
                  :model-value="test.value"
                  :label="t('modules.interfaces.conditions.value')"
                  :min="0"
                  :max="
                    test.field === ConditionField.Active
                      ? 1
                      : test.field === ConditionField.Progress
                        ? 100
                        : test.field === ConditionField.Stacks
                          ? 999
                          : 3600
                  "
                  @update:model-value="check(index, at, { value: $event })"
                />
              </UiField>

              <UiButton
                :disabled="item.checks.length === 1"
                :aria-label="t('modules.interfaces.conditions.removeCheck')"
                @click="
                  rule(index, { checks: item.checks.filter((_, position) => position !== at) })
                "
                >×</UiButton
              >
            </div>
            <div v-if="test.field === ConditionField.Custom" class="condition-data-fields">
              <UiField :label="t('modules.interfaces.conditions.dataField')">
                <FieldPicker
                  :model-value="test.path ?? ''"
                  :formulas="formulas"
                  :rules="triggerRules"
                  @choose="selectPath(index, at, $event)"
                />
              </UiField>
            </div>
          </div>
          <UiButton
            class="condition-add-action is-add"
            :disabled="item.checks.length >= 8"
            @click="
              rule(index, {
                checks: [
                  ...item.checks,
                  { field: ConditionField.Remaining, operator: ComparisonOperator.Less, value: 3 },
                ],
              })
            "
            >＋ {{ t('modules.interfaces.conditions.check') }}</UiButton
          >
        </div>
      </div>
      <div class="condition-group">
        <div class="condition-intro">
          <h3>{{ t('modules.interfaces.conditions.then') }}</h3>
          <p>{{ t('modules.interfaces.redesign.hints.changes') }}</p>
        </div>
        <div class="condition-settings">
          <div
            v-for="(effect, at) in item.changes"
            :key="at"
            class="condition-change"
            :class="{
              'is-expanded': [
                ConditionProperty.Sound,
                ConditionProperty.AnimationConfig,
                ConditionProperty.Text,
                ConditionProperty.IconLabel,
              ].includes(effect.property),
            }"
          >
            <UiField :label="t('modules.interfaces.conditions.property')">
              <UiSelect
                :model-value="effect.property"
                :options="properties"
                :label="t('modules.interfaces.conditions.property')"
                @update:model-value="change(index, at, conditionChanges.create($event))"
              />
            </UiField>
            <UiField
              class="condition-effect-value"
              :label="t('modules.interfaces.conditions.value')"
            >
              <ConditionEffectFields
                :change="effect"
                :widget="widget"
                :session="session"
                @change="change(index, at, $event)"
              />
            </UiField>
            <UiButton
              :aria-label="t('modules.interfaces.conditions.removeChange')"
              :disabled="item.changes.length === 1"
              @click="
                rule(index, { changes: item.changes.filter((_, position) => position !== at) })
              "
              >×</UiButton
            >
          </div>
          <UiButton
            class="condition-add-action is-add"
            :disabled="item.changes.length >= 32"
            @click="
              rule(index, {
                changes: [...item.changes, conditionChanges.create(ConditionProperty.Glow)],
              })
            "
            >＋ {{ t('modules.interfaces.conditions.change') }}</UiButton
          >
        </div>
      </div>
    </section>
  </section>
</template>
<style scoped>
.conditions-editor {
  gap: 20px;
}
.conditions-tools,
.condition-card > header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 10px;
}
.condition-card {
  display: grid;
  gap: 14px;
  min-width: 0;
  padding: 0;
}
.condition-card > header {
  padding-left: var(--editor-section-label, 210px);
}
.condition-group {
  display: grid;
  grid-template-columns: var(--editor-section-label, 210px) minmax(0, 1fr);
  gap: 0;
  min-width: 0;
}
.condition-intro {
  padding-right: 26px;
}
.condition-intro p {
  margin: 6px 0 0;
  color: var(--prism-muted);
  font-size: 11px;
  line-height: 1.65;
}
.condition-settings {
  container-type: inline-size;
  display: flex;
  flex-direction: column;
  gap: 14px;
  min-width: 0;
}
.condition-mode {
  width: 100%;
  max-width: 280px;
}
.condition-preview {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
  padding: 16px;
  border: 1px solid var(--prism-border);
  border-radius: var(--prism-radius-card);
  background: var(--prism-surface);
}
.condition-preview > div {
  display: grid;
  gap: 8px;
}
.condition-check {
  display: grid;
  gap: 12px;
  min-width: 0;
}
.condition-check + .condition-check {
  border-top: 1px solid var(--prism-border);
  padding-top: 14px;
}
.condition-check-fields {
  display: grid;
  grid-template-columns: minmax(0, 1.3fr) minmax(0, 0.8fr) minmax(0, 1fr) 24px;
  align-items: end;
  gap: 12px;
}
.condition-check-fields > .ui-button,
.condition-change > .ui-button {
  width: 24px;
  min-width: 24px;
  height: 28px;
  padding: 0;
  border-color: transparent;
  background: transparent;
}
.condition-data-fields {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}
.condition-manual-path {
  align-self: end;
  min-width: 0;
}
.condition-manual-path > summary {
  padding: 8px 0;
  color: var(--prism-muted);
  cursor: pointer;
}
.condition-change {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) 24px;
  align-items: end;
  gap: 12px;
}
.condition-effect-value :deep(.ui-switch) {
  min-height: 28px;
}
.condition-change.is-expanded {
  grid-template-columns: minmax(0, 1fr) 24px;
}
.condition-change.is-expanded > .condition-effect-value {
  grid-column: 1 / -1;
  grid-row: 2;
}
.condition-change.is-expanded > .ui-button {
  grid-column: 2;
  grid-row: 1;
}
.condition-remove-rule {
  min-height: 24px;
  padding: 0 6px;
  border-color: transparent;
  background: transparent;
  font-size: 12px;
}
.condition-add-action {
  justify-self: start;
  border-color: transparent;
  background: #ffffff03;
  color: var(--prism-muted);
  font-size: 12px;
}
@container (max-width: 540px) {
  .condition-group {
    grid-template-columns: minmax(0, 1fr);
    gap: 14px;
  }
  .condition-card > header {
    padding-left: 0;
  }
}
@container (max-width: 520px) {
  .condition-check-fields {
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr) 24px;
  }
  .condition-check-fields > :first-child {
    grid-column: 1 / 3;
  }
  .condition-check-fields > :last-child {
    grid-column: 3;
    grid-row: 1;
  }
  .condition-change {
    grid-template-columns: minmax(0, 1fr) 24px;
  }
  .condition-change > .condition-effect-value {
    grid-column: 1 / -1;
    grid-row: 2;
  }
  .condition-change > .ui-button {
    grid-column: 2;
    grid-row: 1;
  }
}
@container (max-width: 400px) {
  .condition-data-fields,
  .condition-preview {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
