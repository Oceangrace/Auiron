<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalChartSeries } from '../model/journal-chart-series';
import type { JournalFormatter } from '../runtime/journal-formatter';
import UiButton from '../../../renderer/components/ui/ui-button.vue';

const props = defineProps<{
  series: readonly JournalChartSeries[];
  duration: number;
  formatter: JournalFormatter;
  stacked?: boolean;
}>();

const emit = defineEmits<{ range: [startMs: number, endMs: number] }>();

const { t } = useI18n();

const hidden = ref<readonly string[]>([]);

const hover = ref<number | null>(null);

const start = ref(0);

const end = ref(1);

const anchor = ref<number | null>(null);

const cursor = ref(0);

const count = computed(() => Math.max(1, ...props.series.map((series) => series.values.length)));

const first = computed(() => Math.min(count.value - 1, Math.floor(start.value * count.value)));

const last = computed(() => Math.max(first.value + 1, Math.ceil(end.value * count.value)));

const shown = computed(() => props.series.filter((series) => !hidden.value.includes(series.label)));

const layers = computed(() =>
  shown.value.map((series, index) => ({
    ...series,
    lower: series.values.map((_, bin) =>
      props.stacked === true
        ? shown.value.slice(0, index).reduce((sum, entry) => sum + (entry.values[bin] ?? 0), 0)
        : 0,
    ),
    upper: series.values.map((value, bin) =>
      props.stacked === true
        ? shown.value.slice(0, index + 1).reduce((sum, entry) => sum + (entry.values[bin] ?? 0), 0)
        : value,
    ),
  })),
);

const peak = computed(() =>
  Math.max(1, ...layers.value.flatMap((series) => series.upper.slice(first.value, last.value))),
);

const bin = computed(() =>
  hover.value === null
    ? null
    : Math.min(last.value - 1, first.value + Math.floor(hover.value * (last.value - first.value))),
);

const path = (values: readonly number[]): string => {
  const slice = values.slice(first.value, last.value);

  const points = slice.length === 1 ? [slice[0] ?? 0, slice[0] ?? 0] : slice;

  return points
    .map(
      (value, index, entries) =>
        String((index * 1000) / Math.max(1, entries.length - 1)) +
        ',' +
        String(180 - (value * 168) / peak.value),
    )
    .join(' ');
};

const position = (event: PointerEvent): number => {
  const bounds = (event.currentTarget as SVGElement).getBoundingClientRect();

  return Math.max(0, Math.min(1, (event.clientX - bounds.left) / bounds.width));
};

const move = (event: PointerEvent): void => {
  cursor.value = position(event);
  hover.value = cursor.value;
};

const down = (event: PointerEvent): void => {
  if (event.button !== 0) {
    return;
  }

  anchor.value = position(event);
  cursor.value = anchor.value;
  (event.currentTarget as SVGElement).setPointerCapture(event.pointerId);
};

const up = (): void => {
  if (anchor.value !== null && Math.abs(anchor.value - cursor.value) > 0.02 === true) {
    const width = end.value - start.value;

    const left = start.value;

    start.value = left + Math.min(anchor.value, cursor.value) * width;
    end.value = left + Math.max(anchor.value, cursor.value) * width;
  }

  anchor.value = null;
  hover.value = null;
};

const reset = (): void => {
  start.value = 0;
  end.value = 1;
};

watch(() => props.duration, reset);
watch(
  [first, last],
  () => {
    emit(
      'range',
      (first.value * props.duration) / count.value,
      (last.value * props.duration) / count.value,
    );
  },
  { immediate: true },
);
</script>
<template>
  <section class="journal-timeline">
    <div class="timeline-toolbar">
      <div class="timeline-legend">
        <button
          v-for="line in series"
          :key="line.label"
          type="button"
          :aria-pressed="!hidden.includes(line.label)"
          :style="{ '--series-color': line.color }"
          @click="
            hidden = hidden.includes(line.label)
              ? hidden.filter((label) => label !== line.label)
              : [...hidden, line.label]
          "
        >
          <i />{{ line.label }}
        </button>
      </div>
      <UiButton v-if="start !== 0 || end !== 1" @click="reset">{{
        t('modules.combatJournal.analysis.resetZoom')
      }}</UiButton>
      <small v-else>{{ t('modules.combatJournal.analysis.zoomHint') }}</small>
    </div>
    <div class="timeline-plot">
      <div class="timeline-y">
        <span>{{ formatter.number(peak, true) }}</span
        ><span>{{ formatter.number(peak / 2, true) }}</span
        ><span>0</span>
      </div>
      <svg
        viewBox="0 0 1000 190"
        preserveAspectRatio="none"
        role="img"
        :aria-label="t('modules.combatJournal.analysis.tabs.timeline')"
        @pointermove="move"
        @pointerleave="hover = null"
        @pointerdown="down"
        @pointerup="up"
        @pointercancel="anchor = null"
      >
        <path d="M0 12H1000 M0 96H1000 M0 180H1000" class="timeline-grid" />
        <g v-for="line in layers" :key="line.label">
          <polygon
            :points="path(line.upper) + ' ' + path(line.lower).split(' ').reverse().join(' ')"
            :fill="line.color"
            :opacity="stacked ? 0.45 : 0.08"
          />
          <polyline :points="path(line.upper)" :stroke="line.color" />
        </g>
        <rect
          v-if="anchor !== null"
          :x="Math.min(anchor, cursor) * 1000"
          y="0"
          :width="Math.abs(anchor - cursor) * 1000"
          height="180"
          fill="var(--prism-accent)"
          opacity="0.2"
        />
        <path v-if="hover !== null" :d="`M${hover * 1000} 0V180`" class="timeline-cursor" />
      </svg>
      <div
        v-if="bin !== null"
        class="timeline-tooltip"
        :style="{ left: String(Math.min(72, (hover ?? 0) * 100)) + '%' }"
      >
        <b
          >{{ formatter.time((bin * duration) / count) }} –
          {{ formatter.time(((bin + 1) * duration) / count) }}</b
        >
        <div v-for="line in shown" :key="line.label">
          <span :style="{ color: line.color }">{{ line.label }}</span
          ><strong>{{ formatter.number(line.values[bin] ?? 0, false) }}</strong>
        </div>
      </div>
    </div>
    <div class="timeline-x">
      <span v-for="tick in 5" :key="tick">{{
        formatter.time(((first + ((last - first) * (tick - 1)) / 4) * duration) / count)
      }}</span>
    </div>
  </section>
</template>
<style scoped>
.journal-timeline {
  border: 1px solid var(--prism-border);
  border-radius: 6px;
  padding: 12px;
  background: var(--prism-field);
}
.timeline-toolbar,
.timeline-legend {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 14px;
}
.timeline-toolbar {
  justify-content: space-between;
  margin-bottom: 12px;
  min-height: 28px;
}
.timeline-toolbar small {
  color: var(--prism-muted);
  font-size: 11px;
}
.timeline-legend button {
  display: flex;
  align-items: center;
  gap: 6px;
  border: 0;
  background: none;
  color: var(--prism-text);
  font: inherit;
  font-size: 11px;
  cursor: pointer;
  padding: 3px;
}
.timeline-legend button[aria-pressed='false'] {
  opacity: 0.35;
}
.timeline-legend i {
  width: 15px;
  height: 3px;
  background: var(--series-color);
}
.timeline-plot {
  position: relative;
  padding-left: 48px;
}
.timeline-plot svg {
  display: block;
  width: 100%;
  height: 190px;
  touch-action: none;
  cursor: crosshair;
}
.timeline-plot polyline {
  fill: none;
  stroke-width: 1.7;
  vector-effect: non-scaling-stroke;
}
.timeline-grid {
  stroke: var(--prism-border);
  fill: none;
}
.timeline-cursor {
  stroke: var(--prism-muted);
  stroke-dasharray: 4;
  fill: none;
}
.timeline-y {
  position: absolute;
  left: 0;
  top: 5px;
  bottom: 5px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  font-size: 10px;
  color: var(--prism-muted);
}
.timeline-x {
  display: flex;
  justify-content: space-between;
  padding-left: 48px;
  font-size: 10px;
  color: var(--prism-muted);
  font-variant-numeric: tabular-nums;
}
.timeline-tooltip {
  position: absolute;
  top: 15px;
  min-width: 190px;
  padding: 10px;
  background: var(--prism-panel-background);
  border: 1px solid var(--prism-border);
  border-radius: 4px;
  box-shadow: 0 5px 18px #0005;
  font-size: 11px;
  pointer-events: none;
  z-index: 1;
}
.timeline-tooltip div {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin-top: 7px;
}
</style>
