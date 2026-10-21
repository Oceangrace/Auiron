<script setup lang="ts">
import type { WidgetNode } from '../model/widget-node';
defineProps<{ widget: WidgetNode }>();
</script>
<template>
  <span class="widget-text" :style="{ whiteSpace: widget.style.wrap ? 'pre-wrap' : 'pre' }">{{
    widget.appearance.text
  }}</span>
</template>
