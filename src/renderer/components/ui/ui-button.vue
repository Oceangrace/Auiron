<script setup lang="ts">
withDefaults(defineProps<{ active?: boolean; danger?: boolean }>(), {
  active: false,
  danger: false,
});
</script>
<template>
  <button
    type="button"
    class="ui-button"
    :aria-pressed="active || undefined"
    :class="{ 'is-active': active, 'is-danger': danger }"
  >
    <slot />
  </button>
</template>
