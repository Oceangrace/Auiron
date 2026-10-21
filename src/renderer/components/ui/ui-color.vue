<script setup lang="ts">
const props = defineProps<{ modelValue: string; label: string }>();

const emit = defineEmits<{ 'update:modelValue': [value: string] }>();

const change = (event: Event): void => {
  if (event.target instanceof HTMLInputElement === false) {
    return;
  }

  const element = event.target;

  if (element.checkValidity() === false) {
    element.value = props.modelValue;
    element.reportValidity();

    return;
  }

  const alpha = props.modelValue.length === 9 ? props.modelValue.slice(7) : '';

  const value = element.type === 'color' ? element.value + alpha : element.value;

  emit('update:modelValue', value.toLowerCase());
};
</script>
<template>
  <div class="ui-color">
    <input
      class="ui-color-swatch"
      type="color"
      :value="modelValue.slice(0, 7)"
      :aria-label="label"
      @change="change"
    />
    <input
      class="ui-color-hex"
      type="text"
      :value="modelValue"
      :aria-label="label"
      pattern="#[0-9a-fA-F]{6}([0-9a-fA-F]{2})?"
      maxlength="9"
      required
      spellcheck="false"
      @change="change"
    />
  </div>
</template>
<style scoped>
.ui-color {
  display: flex;
  align-items: center;
  gap: 0.46rem;
  min-height: 30px;
  padding: 4px 6px;
  border: 1px solid transparent;
  border-radius: var(--prism-radius-control);
  background: var(--prism-field);
}
.ui-color:focus-within {
  border-color: var(--prism-accent);
}
.ui-color-swatch {
  width: 1.385rem;
  height: 1.385rem;
  flex-shrink: 0;
  padding: 0;
  border: 1px solid var(--prism-field-border);
  border-radius: 2px;
  background: transparent;
  cursor: pointer;
}
.ui-color-swatch::-webkit-color-swatch-wrapper {
  padding: 0;
}
.ui-color-swatch::-webkit-color-swatch {
  border: 0;
}
.ui-color-hex {
  min-width: 0;
  width: 7rem;
  padding: 0;
  border: 0;
  color: var(--prism-label);
  background: transparent;
  font-family: monospace;
  font-size: 0.923rem;
}
</style>
