<script setup lang="ts">
withDefaults(defineProps<{ label: string; active?: boolean; disabled?: boolean }>(), {
  active: false,
  disabled: false,
});
const emit = defineEmits<{ click: [event: MouseEvent] }>();
</script>

<template>
  <button
    type="button"
    class="ui-icon-button"
    :class="{ 'ui-icon-button-active': active }"
    :aria-label="label"
    :title="label"
    :disabled="disabled"
    @click="emit('click', $event)"
  >
    <slot />
  </button>
</template>
