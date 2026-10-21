<script setup lang="ts">
import { computed } from 'vue';
import type { WidgetNode } from '../model/widget-node';
import type { PreviewVisual } from '../model/preview-visual';
import { WidgetKind } from '../model/widget-kind';
import { WidgetAnimation } from '../model/widget-animation';
import { GroupEffectLayout } from './group-effect-layout';
import { WidgetGeometry } from './widget-geometry';
import IconWidget from './icon-widget.vue';
const props = defineProps<{
  widget: WidgetNode;
  runtime: PreviewVisual | null;
  ticking: boolean;
  sampledAt?: number | null;
}>();

const geometry = new WidgetGeometry();

const layout = new GroupEffectLayout();

const icons = computed(() =>
  (props.runtime?.instances ?? []).map((entry, index, entries) => {
    const widget: WidgetNode = {
      ...props.widget,
      kind: WidgetKind.Icon,
      appearance: {
        ...props.widget.appearance,
        text: '',
        customImage: null,
        iconPath: entry.visual.appearance.iconPath,
        opacity: 1,
      },
      style: { ...props.widget.style, animation: WidgetAnimation.None },
      layout: layout.resolve(props.widget, entries.length, index),
    };

    return {
      key: entry.key,
      widget,
      visual: entry.visual,
      style: geometry.style(widget, widget.layout),
    };
  }),
);
</script>
<template>
  <div
    v-for="entry in icons"
    :key="entry.key"
    class="scene-widget effect-group-icon"
    :style="entry.style"
    :data-effect-key="entry.key"
  >
    <IconWidget
      :widget="entry.widget"
      :runtime="entry.visual"
      :ticking="ticking"
      :sampled-at="sampledAt ?? null"
    />
    <span
      class="widget-decoration"
      :style="{
        border:
          String(entry.widget.style.borderWidth) +
          'px ' +
          entry.widget.style.borderStyle +
          ' ' +
          entry.widget.appearance.borderColor,
        boxSizing: 'border-box',
      }"
    />
  </div>
</template>
