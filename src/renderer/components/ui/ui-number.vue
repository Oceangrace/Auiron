<script setup lang="ts">
import { computed, ref } from 'vue';
const props = defineProps<{
  modelValue: number;
  label: string;
  min?: number;
  max?: number;
  step?: number;
}>();

const focused = ref(false);

const draft = ref('');

const shown = computed(() => (focused.value ? draft.value : String(props.modelValue)));

const focus = (): void => {
  draft.value = String(props.modelValue);
  focused.value = true;
};

const typing = (event: Event): void => {
  if (event.target instanceof HTMLInputElement === true) {
    draft.value = event.target.value;
  }
};

const emit = defineEmits<{ 'update:modelValue': [value: number] }>();

const change = (event: Event): void => {
  if (event.target instanceof HTMLInputElement === false) {
    return;
  }

  if (
    event.target.checkValidity() === false ||
    Number.isFinite(event.target.valueAsNumber) === false
  ) {
    event.target.reportValidity();
    event.target.value = String(props.modelValue);
    draft.value = String(props.modelValue);

    return;
  }

  emit('update:modelValue', event.target.valueAsNumber);
};
</script>
<template>
  <input
    class="ui-input"
    type="number"
    :value="shown"
    :aria-label="label"
    :min="min"
    :max="max"
    :step="step ?? 1"
    required
    @focus="focus"
    @input="typing"
    @blur="focused = false"
    @change="change"
  />
</template>
