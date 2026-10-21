<script setup lang="ts">
defineProps<{ modelValue: boolean; label: string }>();
const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>();
</script>

<template>
  <button
    type="button"
    class="ui-switch"
    role="switch"
    :aria-checked="modelValue"
    @click="emit('update:modelValue', !modelValue)"
  >
    <span class="ui-switch-track" aria-hidden="true" />
    <span class="ui-switch-label">{{ label }}</span>
  </button>
</template>

<style scoped>
.ui-switch {
  display: inline-flex;
  min-height: 1.846rem;
  align-items: center;
  gap: 0.46rem;
  padding: 0;
  border: 0;
  color: var(--prism-label);
  background: transparent;
  cursor: pointer;
}
.ui-switch-label {
  min-width: 0;
  text-align: left;
}
.ui-switch-track {
  display: flex;
  align-items: center;
  width: 30px;
  height: 18px;
  flex-shrink: 0;
  padding: 2px;
  border: 1px solid var(--prism-field-border);
  border-radius: 10px;
  background: var(--prism-field);
}
.ui-switch-track::after {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--prism-label);
  content: '';
  transition: transform 120ms ease;
}
.ui-switch:hover .ui-switch-track,
.ui-switch:focus-visible .ui-switch-track {
  border-color: var(--prism-accent);
}
.ui-switch[aria-checked='true'] {
  color: var(--prism-text);
}
.ui-switch[aria-checked='true'] .ui-switch-track {
  border-color: var(--prism-accent);
  background: var(--prism-accent);
}
.ui-switch[aria-checked='true'] .ui-switch-track::after {
  transform: translateX(12px);
  background: #fff;
}
@media (prefers-reduced-motion: reduce) {
  .ui-switch-track::after {
    transition: none;
  }
}
</style>
