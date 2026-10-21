<script setup lang="ts">
import type { TextPosition } from '../model/text-position';
defineProps<{ position: TextPosition; text: string; singleLine?: boolean }>();
</script>
<template>
  <span
    class="positioned-text"
    :class="['text-position-' + position, { 'single-line': singleLine }]"
    >{{ text }}</span
  >
</template>
<style scoped>
.positioned-text {
  position: absolute;
  z-index: 3;
  pointer-events: none;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  max-width: 100%;
  line-height: 1.1;
}
.single-line {
  width: max-content;
  white-space: nowrap;
  overflow-wrap: normal;
  font-variant-numeric: tabular-nums;
}
.text-position-top-left {
  left: 2px;
  top: 2px;
}
.text-position-top {
  left: 50%;
  top: 2px;
  translate: -50% 0;
}
.text-position-top-right {
  right: 2px;
  top: 2px;
}
.text-position-left {
  left: 2px;
  top: 50%;
  translate: 0 -50%;
}
.text-position-center {
  left: 50%;
  top: 50%;
  translate: -50% -50%;
}
.text-position-right {
  right: 2px;
  top: 50%;
  translate: 0 -50%;
}
.text-position-bottom-left {
  left: 2px;
  bottom: 2px;
}
.text-position-bottom {
  left: 50%;
  bottom: 2px;
  translate: -50% 0;
}
.text-position-bottom-right {
  right: 2px;
  bottom: 2px;
}
.text-position-outside-left {
  right: calc(100% + 4px);
  top: 50%;
  translate: 0 -50%;
  white-space: nowrap;
  max-width: none;
}
.text-position-outside-right {
  left: calc(100% + 4px);
  top: 50%;
  translate: 0 -50%;
  white-space: nowrap;
  max-width: none;
}
</style>
