<script setup lang="ts">
defineProps<{ label: string; hint?: string }>();
</script>
<template>
  <div class="ui-field">
    <span class="ui-label">{{ label }}</span
    ><slot /><small v-if="hint">{{ hint }}</small>
  </div>
</template>
