<script setup lang="ts">
import { ref, watch } from 'vue';
const props = withDefaults(
  defineProps<{
    modelValue: number;
    label: string;
    min?: number;
    max?: number;
    step?: number;
    disabled?: boolean;
  }>(),
  { min: 0, max: 100, step: 1, disabled: false },
);

const emit = defineEmits<{
  'update:modelValue': [value: number];
  preview: [value: number];
}>();

const draft = ref(props.modelValue);

watch(
  () => props.modelValue,
  (value) => {
    draft.value = value;
  },
);

const preview = (event: Event): void => {
  if (event.target instanceof HTMLInputElement === true) {
    draft.value = event.target.valueAsNumber;
    emit('preview', event.target.valueAsNumber);
  }
};

const change = (event: Event): void => {
  if (event.target instanceof HTMLInputElement === false) {
    return;
  }

  if (
    event.target.checkValidity() === false ||
    Number.isFinite(event.target.valueAsNumber) === false
  ) {
    draft.value = props.modelValue;
    event.target.value = String(props.modelValue);

    return;
  }

  draft.value = event.target.valueAsNumber;
  emit('update:modelValue', draft.value);
};
</script>

<template>
  <span class="ui-slider">
    <input
      type="range"
      :value="draft"
      :min="min"
      :max="max"
      :step="step"
      :disabled="disabled"
      :aria-label="label"
      @input="preview"
      @change="change"
    />
    <span class="ui-slider-values"
      ><input
        type="number"
        :value="draft"
        :min="min"
        :max="max"
        :step="step"
        :disabled="disabled"
        :aria-label="label"
        required
        @change="change"
    /></span>
  </span>
</template>
<style scoped>
.ui-slider {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 4.923rem;
  align-items: center;
  gap: 0.615rem;
  min-width: 0;
}
.ui-slider > input {
  appearance: none;
  width: 100%;
  height: 4px;
  margin: 0;
  background: var(--prism-field);
  border: 0;
  border-radius: 2px;
  cursor: pointer;
}
.ui-slider > input::-webkit-slider-thumb {
  appearance: none;
  width: 12px;
  height: 12px;
  border: 0;
  border-radius: 50%;
  background: var(--prism-accent);
}
.ui-slider-values {
  font-family: monospace;
  font-size: 0.885rem;
  color: var(--prism-dim);
}
.ui-slider-values input {
  width: 100%;
  height: 30px;
  appearance: textfield;
  padding: 0 0.23rem;
  border: 1px solid transparent;
  color: var(--prism-text);
  background: var(--prism-field);
  text-align: center;
}
.ui-slider-values input:focus {
  border-color: var(--prism-accent);
}
.ui-slider-values input::-webkit-inner-spin-button {
  appearance: none;
}
</style>
