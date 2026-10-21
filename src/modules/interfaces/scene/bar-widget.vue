<script setup lang="ts">
import { inject, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { PreviewVisual } from '../model/preview-visual';
import { BarIconAnchor } from '../model/bar-icon-anchor';
import { IconAssets } from '../../../renderer/assets/icon-assets';
import { iconAssetsContextKey } from '../../../renderer/assets/icon-assets-context-key';
import { BarHealingLayout } from './bar-healing-layout';
import { BarSurfaceStyle } from './bar-surface-style';
const props = defineProps<{ widget: WidgetNode; runtime?: PreviewVisual | null }>();

const assets = inject(iconAssetsContextKey, () => new IconAssets(), true);

const { t } = useI18n();

const healingLayout = new BarHealingLayout();

const surfaceStyle = new BarSurfaceStyle();

const prediction = computed(() => props.runtime?.healingPrediction ?? null);

const rightText = computed(() => {
  if (
    props.runtime?.knownProgress === false &&
    props.widget.style.barLossColor !== null &&
    props.widget.style.rightText === ''
  ) {
    return t('modules.interfaces.data.unknown');
  }

  if (props.runtime?.knownProgress === false && props.widget.style.rightText === '?') {
    return t('modules.interfaces.data.unknown');
  }

  return props.widget.style.rightText;
});

const direction = computed(() => {
  const style = props.widget.style;

  if (style.barDirection !== null) {
    const edges: Readonly<Record<string, string>> = {
      right: 'left',
      left: 'right',
      up: 'bottom',
      down: 'top',
    };

    return edges[style.barDirection] ?? 'left';
  }

  if (style.barVertical === true) {
    return style.barInverse ? 'top' : 'bottom';
  }

  return style.barInverse ? 'right' : 'left';
});

const inverted = computed(
  () => props.widget.style.barDirection !== null && props.widget.style.barInverse,
);

const mark = (fraction: number): Record<string, string> => ({
  [direction.value]: String((inverted.value ? 1 - fraction : fraction) * 100) + '%',
  ...(props.widget.style.barVertical
    ? { left: '0', right: '0', height: '1px' }
    : { top: '0', bottom: '0', width: '1px' }),
});

const fill = computed(() => {
  const style = props.widget.style;

  const value = inverted.value ? 100 - style.barValue : style.barValue;

  return {
    [direction.value]: '0',
    width: style.barVertical ? '100%' : String(value) + '%',
    height: style.barVertical ? String(value) + '%' : '100%',
    background:
      style.gradient === null
        ? style.fillColor
        : 'linear-gradient(to ' +
          (style.barVertical ? 'top' : 'right') +
          ', ' +
          style.fillColor +
          ', ' +
          style.gradient +
          ')',
    borderRadius: String(style.radius) + 'px',
    transition:
      'width ' +
      String(style.transitionMs) +
      'ms linear, height ' +
      String(style.transitionMs) +
      'ms linear',
  };
});

const image = computed(
  () => props.widget.appearance.customImage ?? assets.url(props.widget.appearance.iconPath),
);

const iconStyle = computed(() => {
  const style = props.widget.style;

  let left = 'calc(50% + ' + String(style.barIconX - style.barIconSize / 2) + 'px)';

  if (style.barIconAnchor === BarIconAnchor.Left) {
    left = String(style.barIconX - style.barIconSize - style.barIconPadding) + 'px';
  }

  if (style.barIconAnchor === BarIconAnchor.Right) {
    left = 'calc(100% + ' + String(style.barIconX + style.barIconPadding) + 'px)';
  }

  return {
    width: String(style.barIconSize) + 'px',
    height: String(style.barIconSize) + 'px',
    left,
    top: 'calc(50% + ' + String(style.barIconY - style.barIconSize / 2) + 'px)',
  };
});
</script>
<template>
  <span class="bar-track" :style="{ borderRadius: String(widget.style.radius) + 'px' }">
    <span class="widget-bar-fill" :style="fill" />
    <span
      v-if="widget.style.barLossColor !== null && runtime?.knownProgress !== false"
      class="bar-loss-fill"
      :style="{
        ...healingLayout.loss(widget.style.barValue, widget.style.barVertical, direction, inverted),
        backgroundColor: widget.style.barLossColor,
      }"
    />
    <template v-if="prediction !== null">
      <span
        class="bar-healing-fill"
        :title="t('modules.interfaces.healing.estimateHint')"
        :style="{
          ...healingLayout.style(prediction, widget.style.barVertical, direction, inverted, false),
          backgroundColor: widget.style.healingColor,
        }"
      />
      <span
        v-if="prediction.overhealPercent > 0"
        class="bar-healing-fill bar-overheal-fill"
        :style="{
          ...healingLayout.style(prediction, widget.style.barVertical, direction, inverted, true),
          ...surfaceStyle.texture(
            widget.style.overhealTexture,
            widget.style.overhealTextureSize,
            widget.style.overhealColor,
          ),
        }"
        :title="t('modules.interfaces.healing.estimateHint')"
      />
    </template>
    <span
      v-if="widget.style.spark"
      class="bar-mark"
      :style="{
        ...mark(widget.style.barValue / 100),
        background: widget.style.sparkColor,
        boxShadow: '0 0 4px ' + widget.style.sparkColor,
      }"
    />
    <span
      v-for="(tick, index) in runtime?.ticks ?? widget.style.ticks.map((value) => value / 100)"
      :key="index"
      class="bar-mark"
      :style="{ ...mark(tick), background: widget.style.tickColor }"
    />
  </span>
  <span class="widget-bar-label" :style="{ whiteSpace: widget.style.wrap ? 'pre-wrap' : 'pre' }">{{
    widget.appearance.text
  }}</span>
  <span v-if="rightText !== ''" class="bar-right-text">{{ rightText }}</span>
  <span
    v-if="runtime?.knownProgress === false && widget.appearance.text === '' && rightText === ''"
    class="bar-no-data"
    :title="t('modules.interfaces.noData')"
    >{{ t('modules.interfaces.data.unknown') }}</span
  >
  <img
    v-if="widget.style.barIconEnabled && image !== null"
    class="bar-icon"
    :style="iconStyle"
    :src="image"
    :alt="widget.name"
    draggable="false"
  />
</template>
<style scoped>
.bar-track {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: inherit;
  justify-content: inherit;
  overflow: hidden;
  pointer-events: none;
}
.widget-bar-fill,
.bar-loss-fill,
.bar-healing-fill,
.bar-mark {
  position: absolute;
  pointer-events: none;
}
.bar-healing-fill {
  transition:
    left 150ms linear,
    right 150ms linear,
    top 150ms linear,
    bottom 150ms linear,
    width 150ms linear,
    height 150ms linear;
}
.widget-bar-label {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: inherit;
  justify-content: inherit;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  overflow: hidden;
  z-index: 1;
  pointer-events: none;
  width: 100%;
  padding: 0 5px;
}
.bar-right-text {
  position: absolute;
  right: 5px;
  top: 50%;
  translate: 0 -50%;
  z-index: 2;
  pointer-events: none;
  white-space: nowrap;
  font-variant-numeric: tabular-nums;
}
.bar-icon {
  position: absolute;
  object-fit: cover;
  pointer-events: none;
}
.bar-no-data {
  position: absolute;
  inset: 0;
  text-align: center;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0.65;
  pointer-events: none;
}
</style>
