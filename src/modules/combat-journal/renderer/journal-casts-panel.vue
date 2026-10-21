<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalAction } from '../model/journal-action';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalFormatter } from '../runtime/journal-formatter';
import { JournalActionKind } from '../model/journal-action-kind';
import JournalSkillIcon from './journal-skill-icon.vue';

const props = defineProps<{
  actions: readonly JournalAction[];
  segment: JournalSegment;
  formatter: JournalFormatter;
  groupActors?: boolean;
  actors?: readonly number[];
  actorFor?: (id: number) => number;
}>();

const emit = defineEmits<{ skill: [id: number | null]; actor: [id: number] }>();

const { t } = useI18n();

const viewport = ref<HTMLElement | null>(null);

const duration = computed(() => Math.max(1000, props.segment.spanMs ?? props.segment.elapsedMs));

const pixelsPerSecond = computed(() => (props.groupActors === true ? 64 : 32));

const trackWidth = computed(() => (duration.value / 1000) * pixelsPerSecond.value + 28);

const tickMs = computed(() => {
  const step = props.groupActors === true ? 1000 : 5000;

  return Math.max(step, Math.ceil(duration.value / 300 / step) * step);
});

const ticks = computed(() =>
  Array.from(
    { length: Math.floor(duration.value / tickMs.value) + 1 },
    (_, index) => index * tickMs.value,
  ),
);

const position = (milliseconds: number): number => (milliseconds / 1000) * pixelsPerSecond.value;

const actorId = (id: number): number => props.actorFor?.(id) ?? id;

const casts = computed(() =>
  props.actions.filter((action) => action.kind === JournalActionKind.Cast),
);

const rows = computed(() => {
  const groups = new Map<number | null, JournalAction[]>();

  if (props.groupActors === true) {
    for (const id of props.actors ?? []) {
      groups.set(id, []);
    }
  }

  for (const action of props.actions.filter(
    (action) =>
      action.kind === JournalActionKind.Cast || action.kind === JournalActionKind.CastStart,
  )) {
    const key = props.groupActors === true ? actorId(action.sourceId) : action.skillId;

    const list = groups.get(key) ?? [];

    list.push(action);
    groups.set(key, list);
  }

  return [...groups]
    .map(([id, events]) => ({
      id,
      events,
      activations: events.filter((event) => event.kind === JournalActionKind.Cast).length,
    }))
    .sort((a, b) => b.events.length - a.events.length);
});

const inTimeline = (events: readonly JournalAction[]): readonly JournalAction[] =>
  events.filter((event) => {
    const at = (event.atUs - props.segment.startedAtUs) / 1000;

    return at + (event.durationMs ?? 0) >= 0 && at <= duration.value;
  });

const left = (event: JournalAction): number =>
  Math.max(0, (event.atUs - props.segment.startedAtUs) / 1000);

const span = (event: JournalAction): number =>
  Math.max(
    0,
    Math.min(
      duration.value,
      (event.atUs - props.segment.startedAtUs) / 1000 + (event.durationMs ?? 0),
    ) - left(event),
  );

const selectRow = (id: number | null): void => {
  if (props.groupActors === true && id !== null) {
    emit('actor', id);

    return;
  }

  emit('skill', id);
};

const selectEvent = (event: JournalAction): void => {
  if (props.groupActors === true) {
    emit('actor', actorId(event.sourceId));

    return;
  }

  emit('skill', event.skillId);
};

watch([() => props.segment.id, () => props.groupActors], () => {
  viewport.value?.scrollTo({ left: 0, top: 0 });
});
</script>
<template>
  <section class="journal-casts-panel" :class="{ 'cast-party': groupActors }">
    <header>
      <span
        >{{ t('modules.combatJournal.analysis.confirmedCasts') }}: <b>{{ casts.length }}</b> ·
        {{ t('modules.combatJournal.analysis.activationsPerMinute') }}
        <b>{{
          segment.elapsedMs > 0
            ? formatter.number((casts.length * 60000) / segment.elapsedMs, false)
            : '—'
        }}</b></span
      >
    </header>
    <p v-if="segment.tracePartial">{{ t('modules.combatJournal.analysis.tracePartial') }}</p>
    <div
      ref="viewport"
      class="cast-scroll"
      role="region"
      tabindex="0"
      :aria-label="t('modules.combatJournal.analysis.confirmedCasts')"
      :style="{
        '--cast-width': String(trackWidth) + 'px',
        '--cast-grid': String(position(tickMs)) + 'px',
      }"
    >
      <div class="cast-ruler">
        <span class="cast-ruler-name" />
        <div class="cast-time-axis">
          <span
            v-for="tick in ticks"
            :key="tick"
            :style="{ left: String(position(tick)) + 'px' }"
            >{{ formatter.time(tick) }}</span
          >
        </div>
      </div>
      <div v-for="row in rows" :key="String(row.id)" class="cast-row">
        <button type="button" class="cast-name" @click="selectRow(row.id)">
          <JournalSkillIcon v-if="!groupActors" :path="formatter.icon(row.id)" /><span
            :style="{
              color: groupActors
                ? formatter.classColor(segment.units[row.id ?? 0]?.className)
                : undefined,
            }"
            >{{ groupActors ? formatter.unit(row.id ?? 0, segment) : formatter.skill(row.id)
            }}<small
              >{{ row.activations }} ·
              {{
                formatter.number((row.activations * 60000) / Math.max(1, segment.elapsedMs), false)
              }}
              /min</small
            ></span
          >
        </button>
        <div class="cast-track">
          <button
            v-for="event in inTimeline(row.events)"
            :key="event.sequence"
            :class="{ preparation: event.kind === JournalActionKind.CastStart }"
            type="button"
            :title="
              formatter.skill(event.skillId) +
              ' · ' +
              formatter.number((event.atUs - segment.startedAtUs) / 1000000, false) +
              's · ' +
              t('modules.combatJournal.analysis.kinds.' + event.kind) +
              (event.durationMs === undefined ? '' : ' · ' + String(event.durationMs) + 'ms') +
              ' · ' +
              formatter.unit(event.sourceId, segment)
            "
            :aria-label="
              formatter.skill(event.skillId) +
              ' ' +
              formatter.time((event.atUs - segment.startedAtUs) / 1000)
            "
            :style="{
              left: String(position(left(event))) + 'px',
              width:
                event.kind === JournalActionKind.CastStart
                  ? String(position(span(event))) + 'px'
                  : undefined,
            }"
            @click="selectEvent(event)"
          >
            <JournalSkillIcon :path="formatter.icon(event.skillId)" />
          </button>
        </div>
      </div>
    </div>
    <p v-if="rows.length === 0">{{ t('modules.combatJournal.analysis.noEvents') }}</p>
  </section>
</template>
<style scoped>
.journal-casts-panel {
  display: flex;
  flex: 1;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
}
header {
  flex-shrink: 0;
  margin-bottom: 16px;
  font-size: 12px;
}
p,
small {
  color: var(--prism-muted);
  font-size: 11px;
}
.cast-scroll {
  overflow: auto;
  min-height: 0;
  border: 1px solid var(--prism-border);
  border-radius: var(--prism-radius-control);
  overscroll-behavior: contain;
  scrollbar-gutter: stable;
}
.cast-scroll:focus-visible {
  outline: 1px solid var(--prism-accent);
  outline-offset: 2px;
}
.cast-row,
.cast-ruler {
  display: grid;
  grid-template-columns: clamp(160px, 22vw, 260px) var(--cast-width);
  align-items: center;
  width: max-content;
  min-width: 100%;
}
.cast-row {
  border-bottom: 1px solid var(--prism-border);
  min-height: 46px;
}
.cast-row:last-child {
  border-bottom: 0;
}
.cast-party .cast-row {
  min-height: 40px;
}
.cast-party .cast-name {
  padding: 4px 12px;
}
.cast-name,
.cast-ruler-name {
  position: sticky;
  left: 0;
  z-index: 2;
  background: var(--prism-background);
  border-right: 1px solid var(--prism-border);
  align-self: stretch;
}
.cast-name {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 7px 12px;
  color: var(--prism-text);
  border-top: 0;
  border-bottom: 0;
  border-left: 0;
  border-radius: 0;
  font: inherit;
  font-size: 12px;
  text-align: left;
  cursor: pointer;
}
.cast-name > span {
  min-width: 0;
  overflow-wrap: anywhere;
}
.cast-name small {
  display: block;
  margin-top: 3px;
}
.cast-track {
  position: relative;
  height: 30px;
  background: repeating-linear-gradient(
    to right,
    var(--prism-border) 0 1px,
    transparent 1px var(--cast-grid)
  );
}
.cast-track button {
  position: absolute;
  top: 2px;
  padding: 0;
  border: 0;
  background: transparent;
  cursor: pointer;
}
.cast-track button.preparation {
  height: 26px;
  min-width: 3px;
  background: color-mix(in srgb, var(--prism-accent) 35%, transparent);
  border: 1px solid var(--prism-accent);
  overflow: hidden;
}
.cast-track button:hover,
.cast-track button:focus-visible {
  z-index: 1;
  outline: 1px solid var(--prism-accent);
}
.cast-ruler {
  position: sticky;
  top: 0;
  z-index: 3;
  background: var(--prism-background);
  border-bottom: 1px solid var(--prism-border);
}
.cast-time-axis {
  position: relative;
  height: 30px;
  font-size: 10px;
  color: var(--prism-muted);
}
.cast-time-axis > span {
  position: absolute;
  top: 8px;
  padding-left: 2px;
}
</style>
