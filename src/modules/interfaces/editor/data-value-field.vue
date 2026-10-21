<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { FormulaDefinition } from '../model/rules/formula-definition';
import type { TriggerRule } from '../model/rules/trigger-rule';
import { FieldCatalog } from './field-catalog';
import { DataValueOptions } from './data-value-options';
import { DataKind } from '../model/rules/data-kind';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
const props = defineProps<{
  path: string;
  modelValue: string;
  formulas: readonly FormulaDefinition[];
  rules: readonly TriggerRule[];
}>();

const emit = defineEmits<{ 'update:modelValue': [value: string] }>();

const { t } = useI18n();

const values = new DataValueOptions(t);

const options = computed(() => values.choices(props.path, props.formulas, props.rules));

const numeric = computed(() =>
  [DataKind.Number, DataKind.Percent, DataKind.Duration, DataKind.Timestamp].includes(
    new FieldCatalog().kind(props.path, props.formulas, props.rules) ?? DataKind.Text,
  ),
);
</script>
<template>
  <UiSelect
    v-if="options.length > 0"
    :model-value="modelValue"
    :options="options"
    :label="t('modules.interfaces.conditions.value')"
    @update:model-value="emit('update:modelValue', $event)"
  />
  <UiNumber
    v-else-if="numeric"
    :model-value="Number(modelValue)"
    :min="-1000000000000"
    :max="1000000000000"
    :label="t('modules.interfaces.conditions.value')"
    @update:model-value="emit('update:modelValue', String($event))"
  />
  <UiInput
    v-else
    :model-value="modelValue"
    :label="t('modules.interfaces.conditions.value')"
    @update:model-value="emit('update:modelValue', $event)"
  />
</template>
