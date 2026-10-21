<script setup lang="ts">
import { WidgetKind } from '../model/widget-kind';
defineProps<{ kind: WidgetKind }>();
const paths: Readonly<Partial<Record<WidgetKind, string>>> = {
  [WidgetKind.Bar]: 'M3 7h18v10H3z M3 7h11v10',
  [WidgetKind.DynamicGroup]: 'M4 4h4v4H4z M4 10h4v4H4z M4 16h4v4H4z M12 6h8 M12 12h8 M12 18h8',
  [WidgetKind.Text]: 'M5 5h14 M12 5v14 M9 19h6',
  [WidgetKind.Icon]: 'M4 4h16v16H4z M7 16l4-5 3 3 2-2 4 5 M8 8h.01',
  [WidgetKind.Group]: 'M4 9V4h5 M15 4h5v5 M20 15v5h-5 M9 20H4v-5 M8 8h8v8H8z',
};
</script>
<template>
  <svg class="widget-kind-icon" viewBox="0 0 24 24" aria-hidden="true">
    <path :d="paths[kind] ?? ''" />
  </svg>
</template>
<style scoped>
.widget-kind-icon {
  width: 1.2rem;
  height: 1.2rem;
  flex-shrink: 0;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.4;
  stroke-linecap: round;
  stroke-linejoin: round;
}
</style>
