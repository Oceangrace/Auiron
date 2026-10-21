<script setup lang="ts">
import { ref } from 'vue';
import type { FormulaDefinition } from '../model/rules/formula-definition';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import FieldPicker from './field-picker.vue';
const props = withDefaults(
  defineProps<{
    modelValue: string;
    label: string;
    multiline?: boolean;
    formulas?: readonly FormulaDefinition[];
  }>(),
  { multiline: false, formulas: () => [] },
);

const emit = defineEmits<{ 'update:modelValue': [value: string] }>();

const field = ref<HTMLElement | null>(null);

const insert = (path: string): void => {
  const input = field.value?.querySelector<HTMLInputElement | HTMLTextAreaElement>(
    'textarea,input',
  );

  const value = input?.value ?? props.modelValue;

  const start = input?.selectionStart ?? value.length;

  const end = input?.selectionEnd ?? value.length;

  emit('update:modelValue', value.slice(0, start) + '{' + path + '}' + value.slice(end));
};
</script>
<template>
  <div ref="field" class="template-field">
    <UiInput
      :model-value="modelValue"
      :label="label"
      :multiline="multiline"
      @update:model-value="emit('update:modelValue', $event)"
    />
    <FieldPicker :formulas="formulas" @choose="insert" />
  </div>
</template>
<style scoped>
.template-field {
  display: grid;
  gap: 6px;
}
</style>
