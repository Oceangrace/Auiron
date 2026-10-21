<script setup lang="ts">
import { computed } from 'vue';
import type { WidgetLayout } from '../model/widget-layout';
import type { Viewport } from '../model/viewport';
const props = defineProps<{ layout: WidgetLayout; measure: WidgetLayout; viewport: Viewport }>();

const width = computed(() => Math.round(props.measure.width));

const height = computed(() => Math.round(props.measure.height));

const widthLabel = computed(() => ({
  left: `${String(Math.max(32, Math.min(props.viewport.width - 32, props.layout.x + props.layout.width / 2)))}px`,
  top: `${String(Math.max(4, props.layout.y - 27))}px`,
}));

const heightLabel = computed(() => ({
  left: `${String(Math.max(4, Math.min(props.viewport.width - 70, props.layout.x + props.layout.width + 8)))}px`,
  top: `${String(Math.max(12, Math.min(props.viewport.height - 12, props.layout.y + props.layout.height / 2)))}px`,
}));
</script>
<template>
  <div class="selection-guides" aria-hidden="true">
    <svg :viewBox="'0 0 ' + viewport.width + ' ' + viewport.height" preserveAspectRatio="none">
      <g class="selection-axes">
        <line
          v-for="x in [layout.x, layout.x + layout.width]"
          :key="'x' + x"
          :x1="x"
          y1="0"
          :x2="x"
          :y2="viewport.height"
        />
        <line
          v-for="y in [layout.y, layout.y + layout.height]"
          :key="'y' + y"
          x1="0"
          :y1="y"
          :x2="viewport.width"
          :y2="y"
        />
      </g>
      <rect
        :x="layout.x"
        :y="layout.y"
        :width="layout.width"
        :height="layout.height"
        class="selection-outline"
      />
    </svg>
    <span class="selection-measure selection-width" :style="widthLabel">{{ width }} px</span>
    <span class="selection-measure selection-height" :style="heightLabel">{{ height }} px</span>
  </div>
</template>
<style scoped>
.selection-guides {
  position: fixed;
  inset: 0;
  z-index: 9999;
  pointer-events: none;
}
.selection-guides svg {
  position: absolute;
  width: 100%;
  height: 100%;
  overflow: visible;
}
.selection-axes {
  fill: none;
  stroke: var(--prism-accent);
  stroke-width: 1;
  stroke-dasharray: 4 4;
  opacity: 0.45;
}
.selection-outline {
  fill: none;
  stroke: var(--prism-accent);
  stroke-width: 1;
}
.selection-measure {
  position: absolute;
  padding: 3px 6px;
  border: 1px solid var(--prism-accent);
  background: #141414ed;
  color: var(--prism-accent);
  font: 11px/1.2 monospace;
  white-space: nowrap;
}
.selection-width {
  transform: translateX(-50%);
}
.selection-height {
  transform: translateY(-50%);
}
</style>
