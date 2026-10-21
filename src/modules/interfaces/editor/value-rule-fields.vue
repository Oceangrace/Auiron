<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { ValueTrigger } from '../model/rules/value-trigger';
import type { FormulaDefinition } from '../model/rules/formula-definition';
import type { TriggerRule } from '../model/rules/trigger-rule';
import { ComparisonOperator } from '../model/rules/comparison-operator';
import { DataValueOptions } from './data-value-options';
import { DataKind } from '../model/rules/data-kind';
import { FieldCatalog } from './field-catalog';
import DataValueField from './data-value-field.vue';
import FieldPicker from './field-picker.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
const props = defineProps<{
  trigger: ValueTrigger;
  formulas: readonly FormulaDefinition[];
  rules: readonly TriggerRule[];
}>();

const emit = defineEmits<{ change: [trigger: ValueTrigger] }>();

const { t } = useI18n();

const catalog = new FieldCatalog();

const boolean = computed(
  () => catalog.kind(props.trigger.path, props.formulas, props.rules) === DataKind.Boolean,
);

const operators = computed(() =>
  Object.values(ComparisonOperator)
    .filter(
      (value) =>
        !boolean.value ||
        value === ComparisonOperator.Equal ||
        value === ComparisonOperator.NotEqual,
    )
    .map((value) => ({ value, label: t('modules.interfaces.rules.operators.' + value) })),
);

const choose = (path: string): void => {
  const kind = catalog.kind(path, props.formulas, props.rules);

  const first = new DataValueOptions(t).choices(path, props.formulas, props.rules)[0]?.value;

  emit('change', {
    ...props.trigger,
    path,
    operator: ComparisonOperator.Equal,
    value: first ?? (kind === DataKind.Text ? '' : '0'),
  });
};
</script>
<template>
  <div class="field-grid value-rule-fields">
    <UiField :label="t('modules.interfaces.conditions.dataField')"
      ><FieldPicker
        :model-value="trigger.path"
        :formulas="formulas"
        :rules="rules"
        @choose="choose"
    /></UiField>
    <UiField :label="t('modules.interfaces.conditions.operator')"
      ><UiSelect
        :model-value="trigger.operator"
        :options="operators"
        :label="t('modules.interfaces.conditions.operator')"
        @update:model-value="emit('change', { ...trigger, operator: $event })"
    /></UiField>
    <UiField :label="t('modules.interfaces.conditions.value')"
      ><DataValueField
        :path="trigger.path"
        :model-value="trigger.value"
        :formulas="formulas"
        :rules="rules"
        @update:model-value="emit('change', { ...trigger, value: $event })"
    /></UiField>
  </div>
</template>
