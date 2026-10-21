<script setup lang="ts">
import type { SnapGuides } from '../editor/snap-guides';
defineProps<{ guides: SnapGuides }>();
</script>
<template>
  <div class="alignment-guides" aria-hidden="true">
    <span v-if="guides.x !== null" class="alignment-vertical" :style="{ left: guides.x + 'px' }" />
    <span v-if="guides.y !== null" class="alignment-horizontal" :style="{ top: guides.y + 'px' }" />
  </div>
</template>
<style scoped>
.alignment-guides {
  position: fixed;
  inset: 0;
  z-index: 10000;
  pointer-events: none;
}
.alignment-guides span {
  position: absolute;
  background: var(--prism-accent);
  box-shadow: 0 0 4px var(--prism-accent);
}
.alignment-vertical {
  top: 0;
  bottom: 0;
  width: 1px;
}
.alignment-horizontal {
  left: 0;
  right: 0;
  height: 1px;
}
</style>
