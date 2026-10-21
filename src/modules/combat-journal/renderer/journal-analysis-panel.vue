<script setup lang="ts">
import { computed, nextTick, reactive, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalWindow } from '../model/journal-window';
import type { JournalQuery } from '../runtime/journal-query';
import type { JournalFormatter } from '../runtime/journal-formatter';
import type { JournalAnalysisRow } from '../model/journal-analysis-row';
import { JournalAnalysis } from '../runtime/journal-analysis';
import { JournalAnalysisTab } from '../model/journal-analysis-tab';
import { JournalAnalysisSort } from '../model/journal-analysis-sort';
import { JournalActionKind } from '../model/journal-action-kind';
import { JournalDetail } from '../model/journal-detail';
import { JournalMetric } from '../model/journal-metric';
import { CombatCounter } from '../model/combat-counter';
import JournalDeathsPanel from './journal-deaths-panel.vue';
import JournalEffectsPanel from './journal-effects-panel.vue';
import JournalOverview from './journal-overview.vue';
import JournalTimeline from './journal-timeline.vue';
import JournalSkillIcon from './journal-skill-icon.vue';
import JournalEventsPanel from './journal-events-panel.vue';
import JournalCastsPanel from './journal-casts-panel.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import { JournalScope } from '../model/journal-scope';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import JournalFightPicker from './journal-fight-picker.vue';
import JournalIntervalPanel from './journal-interval-panel.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiIconButton from '../../../renderer/components/ui/ui-icon-button.vue';
import { JournalNavigation } from '../runtime/journal-navigation';
import type { JournalNavigationState } from '../model/journal-navigation-state';

const props = defineProps<{
  segment: JournalSegment;
  segments: readonly JournalSegment[];
  actor: number;
  window: JournalWindow;
  query: JournalQuery;
  formatter: JournalFormatter;
  timelineAvailable: boolean;
  exportFile: (text: string) => Promise<void>;
}>();

const selectedFight = ref<string | null>(null);

const segment = computed(
  () => props.segments.find((fight) => fight.id === selectedFight.value) ?? props.segment,
);

const timelineAvailable = computed(() => selectedFight.value !== null || props.timelineAvailable);

const scope = ref(props.window.scope);

const mergeSummons = ref(props.window.mergeSummons);

const { t } = useI18n();

const analysis = new JournalAnalysis();

const tab = ref(JournalAnalysisTab.Overview);

let restoring = false;

const quantitative = computed(() =>
  [
    JournalAnalysisTab.Skills,
    JournalAnalysisTab.Targets,
    JournalAnalysisTab.Timeline,
    JournalAnalysisTab.Events,
    JournalAnalysisTab.Compare,
  ].includes(tab.value),
);

const scroll = ref<HTMLElement | null>(null);

const actor = ref(props.actor);

const metric = ref(
  new Map([
    [JournalMetric.DamageTaken, JournalMetric.DamageTaken],
    [JournalMetric.HealingTaken, JournalMetric.HealingTaken],
    [JournalMetric.Healing, JournalMetric.Healing],
    [JournalMetric.Hps, JournalMetric.Healing],
  ]).get(props.window.metric) ?? JournalMetric.Damage,
);

const bySkill = ref(false);

const raidCasts = ref(true);

const castActor = (id: number): number => analysis.root(id, segment.value, mergeSummons.value);

const castActors = computed(() => [
  ...new Set(props.query.participants(segment.value, scope.value).map(castActor)),
]);

const groupActions = computed(() => {
  const ids = new Set(castActors.value);

  return (segment.value.actions ?? []).filter((action) => ids.has(castActor(action.sourceId)));
});

const interval = ref<readonly [number, number]>([
  0,
  segment.value.spanMs ?? segment.value.elapsedMs,
]);

const analysisWindow = computed(() => ({
  ...props.window,
  metric: metric.value,
  scope: scope.value,
  mergeSummons: mergeSummons.value,
  limit: Number.MAX_SAFE_INTEGER,
}));

const metricOptions = computed(() =>
  [
    JournalMetric.Damage,
    JournalMetric.Healing,
    JournalMetric.DamageTaken,
    JournalMetric.HealingTaken,
  ].map((value) => ({ value, label: t('modules.combatJournal.metrics.' + value) })),
);

const actorOptions = computed(() =>
  [
    ...new Set([
      actor.value,
      ...(segment.value.actions ?? []).map((action) =>
        analysis.root(action.sourceId, segment.value, mergeSummons.value),
      ),
      ...props.query
        .rows(segment.value, { ...analysisWindow.value, scope: JournalScope.All })
        .flatMap((row) => (row.actorId === null ? [] : [row.actorId])),
    ]),
  ].map((id) => ({ value: String(id), label: props.formatter.unit(id, segment.value) })),
);

const selectActor = (id: number, value: JournalMetric): void => {
  actor.value = id;
  metric.value = value;
  tab.value = JournalAnalysisTab.Skills;
};

const selectDeath = (id: number): void => {
  selectActor(id, JournalMetric.DamageTaken);
  tab.value = JournalAnalysisTab.Deaths;
};

// Time-based tabs need a single fight; never keep one open for a combined period.
watch(
  () => timelineAvailable.value,
  (available) => {
    const timed = [
      JournalAnalysisTab.Timeline,
      JournalAnalysisTab.Rotation,
      JournalAnalysisTab.Effects,
      JournalAnalysisTab.Deaths,
    ];

    if (available === false && timed.includes(tab.value) === true) {
      tab.value = JournalAnalysisTab.Skills;
    }
  },
);

const sortMode = ref(JournalAnalysisSort.Amount);

const search = ref('');

const target = ref('all');

const skill = ref<string | null>(null);

const reference = ref('none');

const referenceActor = ref('');

const exportError = ref('');

const incoming = computed(
  () => metric.value === JournalMetric.DamageTaken || metric.value === JournalMetric.HealingTaken,
);

const healing = computed(() => analysis.healing(metric.value));

const amountMetric = computed(() => (healing.value ? JournalMetric.Healing : JournalMetric.Damage));

const amountWindow = analysisWindow;

const chooseActor = (id: number): void => {
  if (
    props.query.rows(segment.value, analysisWindow.value).some((row) => row.actorId === id) ===
    false
  ) {
    scope.value = JournalScope.All;
  }

  actor.value = id;
};

const targetRows = computed(() =>
  props.query.rows(segment.value, amountWindow.value, JournalDetail.Targets, actor.value),
);

const targetOptions = computed(() => [
  { value: 'all', label: t('modules.combatJournal.analysis.allTargets') },
  ...targetRows.value.map((row) => ({ value: row.id, label: row.label })),
]);

const filtered = computed<JournalSegment>(() => ({
  ...segment.value,
  records: segment.value.records.filter(
    (record) =>
      target.value === 'all' ||
      String(incoming.value ? record.sourceId : record.recipientId) === target.value,
  ),
}));

const actorRow = computed(
  () =>
    props.query
      .rows(filtered.value, amountWindow.value)
      .find((row) => row.actorId === actor.value) ?? null,
);

const summary = computed(() =>
  actorRow.value === null
    ? null
    : analysis.row(actorRow.value, metric.value, segment.value.elapsedMs),
);

const entries = computed(() =>
  props.query
    .rows(filtered.value, amountWindow.value, JournalDetail.Skills, actor.value)
    .map((row) => analysis.row(row, metric.value, segment.value.elapsedMs)),
);

const visible = computed(() =>
  analysis.ordered(
    entries.value.filter((entry) =>
      entry.row.label.toLocaleLowerCase().includes(search.value.toLocaleLowerCase()),
    ),
    sortMode.value,
  ),
);

const selected = computed(
  () => entries.value.find((entry) => entry.row.id === skill.value) ?? null,
);

const actions = computed(() =>
  analysis.actions(
    segment.value,
    actor.value,
    mergeSummons.value,
    incoming.value,
    target.value === 'all' ? null : Number(target.value),
  ),
);

const casts = computed(() =>
  actions.value.filter((action) => action.kind === JournalActionKind.Cast),
);

const castCounts = computed(() => {
  const counts = new Map<string, number>();

  for (const action of casts.value) {
    const key = String(action.skillId);

    counts.set(key, (counts.get(key) ?? 0) + 1);
  }

  return counts;
});

const castCount = (id: string): number | null =>
  full.value && target.value === 'all' && !incoming.value ? (castCounts.value.get(id) ?? 0) : null;

const eventActions = computed(() =>
  actions.value.filter((action) => skill.value === null || String(action.skillId) === skill.value),
);

const rotationActions = computed(() =>
  analysis
    .actions(
      segment.value,
      actor.value,
      mergeSummons.value,
      false,
      target.value === 'all' ? null : Number(target.value),
    )
    .filter((action) => skill.value === null || String(action.skillId) === skill.value),
);

const timelineEvents = computed(() =>
  eventActions.value.filter((action) => {
    const at = (action.atUs - segment.value.startedAtUs) / 1000;

    return (
      at >= interval.value[0] &&
      (at < interval.value[1] ||
        (interval.value[1] >= (segment.value.spanMs ?? segment.value.elapsedMs) &&
          at === interval.value[1]))
    );
  }),
);

const abilitySeries = computed(() => {
  const top = entries.value.slice(0, 5);

  const ids = new Set(top.map((entry) => entry.row.id));

  const groups = [
    ...top.map((entry) => ({ id: entry.row.id, label: entry.row.label })),
    { id: 'rest', label: t('modules.combatJournal.analysis.otherSkills') },
  ];

  return groups.map((group, index) => ({
    label: group.label,
    color: ['#83b7ed', '#b599d9', '#7bc99f', '#e6bb7a', '#dc8888', '#8b9da2'][index] ?? '#8b9da2',
    values: analysis.chart(
      eventActions.value.filter((action) =>
        group.id === 'rest'
          ? !ids.has(String(action.skillId))
          : String(action.skillId) === group.id,
      ),
      healing.value ? JournalActionKind.Healing : JournalActionKind.Damage,
      segment.value.spanMs ?? segment.value.elapsedMs,
      segment.value.startedAtUs,
    ),
  }));
});

const chartSeries = computed(() => [
  {
    label:
      t(
        'modules.combatJournal.metrics.' +
          (incoming.value ? JournalMetric.DamageTaken : JournalMetric.Damage),
      ) + '/s',
    color: '#83b7ed',
    values: damage.value,
  },
  {
    label:
      t(
        'modules.combatJournal.metrics.' +
          (incoming.value ? JournalMetric.HealingTaken : JournalMetric.Healing),
      ) + '/s',
    color: '#7bc99f',
    values: heals.value,
  },
]);

const criticalCounters = [
  CombatCounter.CriticalHits,
  CombatCounter.PerfectHits,
  CombatCounter.DoubleHits,
  CombatCounter.BackHits,
  CombatCounter.FrontalHits,
  CombatCounter.ParryHits,
  CombatCounter.ShieldBlockHits,
  CombatCounter.AdditionalHitEvents,
];

const flags = computed(() =>
  criticalCounters.map((counter) => ({
    counter,
    value: selected.value === null ? null : analysis.ratio(selected.value.row, counter),
  })),
);

const otherCounters = computed(() =>
  Object.values(CombatCounter)
    .map((counter) => ({ counter, value: selected.value?.row.statistics.counters[counter] ?? 0 }))
    .filter((entry) => entry.value !== 0),
);

const full = computed(
  () => segment.value.tracePartial !== true && (segment.value.actions?.length ?? 0) !== 0,
);

const damage = computed(() =>
  analysis.chart(
    eventActions.value,
    JournalActionKind.Damage,
    segment.value.spanMs ?? segment.value.elapsedMs,
    segment.value.startedAtUs,
  ),
);

const heals = computed(() =>
  analysis.chart(
    eventActions.value,
    JournalActionKind.Healing,
    segment.value.spanMs ?? segment.value.elapsedMs,
    segment.value.startedAtUs,
  ),
);

const compareOptions = computed(() => [
  { value: 'none', label: t('modules.combatJournal.analysis.chooseFight') },
  ...props.segments
    .filter((entry) => entry.id !== segment.value.id && entry.endedAtUs !== null)
    .map((entry) => ({
      value: entry.id,
      label:
        props.formatter.npc(entry.npcId) +
        ' · ' +
        props.formatter.time(entry.elapsedMs) +
        ' · ' +
        new Date(entry.startedAtUs / 1000).toLocaleString(),
    })),
]);

const other = computed(() => props.segments.find((entry) => entry.id === reference.value) ?? null);

const otherActors = computed(() => props.query.rows(other.value, amountWindow.value));

const otherActorOptions = computed(() =>
  otherActors.value.map((row) => ({ value: String(row.actorId), label: row.label })),
);

const otherEntries = computed(() =>
  props.query
    .rows(other.value, amountWindow.value, JournalDetail.Skills, Number(referenceActor.value))
    .map((row) => analysis.row(row, metric.value, other.value?.elapsedMs ?? 0)),
);

const compared = computed(() => {
  const current = new Map(entries.value.map((entry) => [entry.row.id, entry]));

  const old = new Map(otherEntries.value.map((entry) => [entry.row.id, entry]));

  return [...new Set([...current.keys(), ...old.keys()])].map((id) => ({
    id,
    current: current.get(id),
    previous: old.get(id),
  }));
});

watch([() => segment.value.id, actor, metric, mergeSummons], () => {
  if (restoring === true) {
    return;
  }

  target.value = 'all';
  skill.value = null;
  reference.value = 'none';
  interval.value = [0, segment.value.spanMs ?? segment.value.elapsedMs];
});
watch(
  () => props.actor,
  (value) => {
    actor.value = value;
  },
);
watch(
  () => segment.value.id,
  () => {
    if (restoring === true) {
      return;
    }

    actor.value = segment.value.playerId ?? props.actor;
    tab.value = JournalAnalysisTab.Overview;
  },
);
watch(reference, () => {
  if (restoring === true) {
    return;
  }

  const same = otherActors.value.find((row) => row.label === actorRow.value?.label);

  referenceActor.value = String(
    same?.actorId ?? other.value?.playerId ?? otherActors.value[0]?.actorId ?? '',
  );
});
watch(tab, (value) => {
  if (restoring === false && value === JournalAnalysisTab.Compare) {
    target.value = 'all';
    skill.value = null;
  }
});
watch(raidCasts, (value) => {
  if (restoring === false && value === true) {
    target.value = 'all';
    skill.value = null;
  }
});
watch(
  [tab, actor, metric, target],
  () => {
    scroll.value?.scrollTo({ top: 0, left: 0 });
  },
  { flush: 'post' },
);
const format = (value: number | null): string =>
  value === null ? '—' : props.formatter.number(value, false);

const percent = (value: number | null): string => (value === null ? '—' : format(value) + '%');

const rateDelta = (
  current: JournalAnalysisRow | undefined,
  previous: JournalAnalysisRow | undefined,
): string => {
  const value = analysis.difference(current, previous);

  if (value === null) {
    return '—';
  }

  return (value > 0 ? '+' : '') + format(value);
};

const exportCsv = async (): Promise<void> => {
  try {
    const header = [
      'skill',
      'amount',
      'share_percent',
      'rate',
      'hit_events',
      'average_event',
      'critical_direct_percent',
      'periodic_amount',
      'complete',
      'activations',
    ];

    const values = visible.value.map((entry) => [
      ...analysis.exportRow(entry),
      castCount(entry.row.id) ?? '',
    ]);

    await props.exportFile(analysis.csv([header, ...values]));
    exportError.value = '';
  } catch (error) {
    exportError.value = String(error);
  }
};

const location = (): JournalNavigationState => ({
  fight: selectedFight.value,
  tab: tab.value,
  actor: actor.value,
  metric: metric.value,
  scope: scope.value,
  mergeSummons: mergeSummons.value,
  target: target.value,
  skill: skill.value,
  reference: reference.value,
  referenceActor: referenceActor.value,
  bySkill: bySkill.value,
  raidCasts: raidCasts.value,
  sort: sortMode.value,
  search: search.value,
  interval: interval.value,
});

const navigation = reactive(new JournalNavigation(location()));

const restore = async (state: JournalNavigationState | null): Promise<void> => {
  if (state === null) {
    return;
  }

  restoring = true;
  selectedFight.value = state.fight;
  tab.value = state.tab;
  actor.value = state.actor;
  metric.value = state.metric;
  scope.value = state.scope;
  mergeSummons.value = state.mergeSummons;
  target.value = state.target;
  skill.value = state.skill;
  reference.value = state.reference;
  referenceActor.value = state.referenceActor;
  bySkill.value = state.bySkill;
  raidCasts.value = state.raidCasts;
  sortMode.value = state.sort;
  search.value = state.search;
  interval.value = state.interval;
  await nextTick();
  restoring = false;
};

watch(
  [
    selectedFight,
    tab,
    actor,
    metric,
    scope,
    mergeSummons,
    target,
    skill,
    reference,
    referenceActor,
    raidCasts,
  ],
  () => {
    if (restoring === false) {
      navigation.visit(location());
    }
  },
  { flush: 'post' },
);
watch(
  [search, sortMode, bySkill, interval],
  () => {
    if (restoring === false) {
      navigation.update(location());
    }
  },
  { flush: 'post' },
);

const navigateKey = (event: KeyboardEvent): void => {
  if (event.altKey === false || ['ArrowLeft', 'ArrowRight'].includes(event.key) === false) {
    return;
  }

  event.preventDefault();
  void restore(event.key === 'ArrowLeft' ? navigation.back() : navigation.forward());
};
</script>
<template>
  <div class="journal-analysis" @keydown="navigateKey">
    <div class="analysis-toolbar">
      <div class="analysis-history">
        <UiIconButton
          :label="t('modules.combatJournal.analysis.back') + ' (Alt+←)'"
          :disabled="!navigation.canBack"
          @click="restore(navigation.back())"
        >
          <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m11 4-6 6 6 6M5 10h11" /></svg>
        </UiIconButton>
        <UiIconButton
          :label="t('modules.combatJournal.analysis.forward') + ' (Alt+→)'"
          :disabled="!navigation.canForward"
          @click="restore(navigation.forward())"
        >
          <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m9 4 6 6-6 6M4 10h11" /></svg>
        </UiIconButton>
      </div>
      <JournalFightPicker
        :segment="segment"
        :segments="segments"
        :formatter="formatter"
        @select="selectedFight = $event"
      />
    </div>
    <div class="analysis-filters analysis-scope">
      <UiSelect
        :label="t('modules.combatJournal.analysis.participants')"
        :model-value="scope"
        :options="
          Object.values(JournalScope).map((value) => ({
            value,
            label: t('modules.combatJournal.scopes.' + value),
          }))
        "
        @update:model-value="scope = $event"
      />
      <UiSwitch v-model="mergeSummons" :label="t('modules.combatJournal.summons')" />
    </div>
    <nav class="analysis-tabs" :aria-label="t('modules.combatJournal.analysis.navigation')">
      <button
        v-for="mode in Object.values(JournalAnalysisTab)"
        :key="mode"
        type="button"
        :aria-pressed="tab === mode"
        :title="!timelineAvailable ? t('modules.combatJournal.analysis.singleCombat') : undefined"
        :disabled="
          !timelineAvailable &&
          [
            JournalAnalysisTab.Timeline,
            JournalAnalysisTab.Rotation,
            JournalAnalysisTab.Effects,
            JournalAnalysisTab.Deaths,
            JournalAnalysisTab.Events,
          ].includes(mode)
        "
        @click="tab = mode"
      >
        {{ t('modules.combatJournal.analysis.tabs.' + mode) }}
      </button>
    </nav>
    <div
      ref="scroll"
      class="analysis-scroll"
      :class="{ 'analysis-scroll-rotation': tab === JournalAnalysisTab.Rotation }"
    >
      <div v-if="tab === JournalAnalysisTab.Rotation" class="analysis-chart-modes cast-view-modes">
        <button type="button" :aria-pressed="raidCasts" @click="raidCasts = true">
          {{ t('modules.combatJournal.analysis.allParticipants') }}
        </button>
        <button type="button" :aria-pressed="!raidCasts" @click="raidCasts = false">
          {{ t('modules.combatJournal.analysis.selectedParticipant') }}
        </button>
      </div>
      <header
        v-if="
          tab !== JournalAnalysisTab.Overview && !(tab === JournalAnalysisTab.Rotation && raidCasts)
        "
        class="analysis-context"
      >
        <div>
          <strong>{{ actorRow?.label ?? formatter.unit(actor, segment) }}</strong
          ><small
            >{{ formatter.time(segment.elapsedMs) }} ·
            {{ t('modules.combatJournal.metrics.' + metric) }}</small
          >
        </div>
        <UiButton
          v-if="tab === JournalAnalysisTab.Skills || tab === JournalAnalysisTab.Compare"
          @click="exportCsv"
          >CSV</UiButton
        >
      </header>
      <div
        v-if="
          tab !== JournalAnalysisTab.Overview && !(tab === JournalAnalysisTab.Rotation && raidCasts)
        "
        class="analysis-filters analysis-selector"
      >
        <UiSelect
          :label="t('modules.combatJournal.analysis.participants')"
          :model-value="String(actor)"
          :options="actorOptions"
          @update:model-value="chooseActor(Number($event))"
        />
        <UiSelect
          v-if="quantitative"
          :label="t('modules.combatJournal.metric')"
          :model-value="metric"
          :options="metricOptions"
          @update:model-value="metric = $event"
        />
      </div>
      <div v-if="summary && quantitative" class="analysis-summary">
        <div>
          <span>{{ t('modules.combatJournal.metrics.' + amountMetric) }}</span
          ><b>{{ summary.row.value === null ? '≥ ' : '' }}{{ format(summary.amount) }}</b>
        </div>
        <div>
          <span>{{ healing ? 'HPS' : 'DPS' }}</span
          ><b>{{ summary.row.value === null ? '≥ ' : '' }}{{ format(summary.rate) }}</b>
        </div>
        <div>
          <span>{{ t('modules.combatJournal.analysis.share') }}</span
          ><b>{{ percent(summary.row.share) }}</b>
        </div>
        <div>
          <span>{{ t('modules.combatJournal.analysis.hitEvents') }}</span
          ><b>{{ format(summary.hits) }}</b>
        </div>
      </div>
      <p v-if="summary?.row.value === null" class="analysis-note">
        {{ t('modules.combatJournal.partial') }}
      </p>
      <div
        v-if="
          tab !== JournalAnalysisTab.Overview &&
          tab !== JournalAnalysisTab.Effects &&
          tab !== JournalAnalysisTab.Deaths &&
          !(tab === JournalAnalysisTab.Rotation && raidCasts)
        "
        class="analysis-filters"
      >
        <UiSelect
          v-if="tab !== JournalAnalysisTab.Compare"
          :label="t('modules.combatJournal.analysis.target')"
          :model-value="target"
          :options="targetOptions"
          @update:model-value="
            target = $event;
            skill = null;
          "
        />
        <input
          v-if="tab === JournalAnalysisTab.Skills || tab === JournalAnalysisTab.Compare"
          v-model="search"
          class="ui-input"
          :aria-label="t('modules.combatJournal.analysis.search')"
          :placeholder="t('modules.combatJournal.analysis.search')"
        />
      </div>
      <div
        v-if="
          skill !== null &&
          tab !== JournalAnalysisTab.Overview &&
          !(tab === JournalAnalysisTab.Rotation && raidCasts)
        "
        class="analysis-active-filter"
      >
        <span>{{ selected?.row.label ?? formatter.skill(Number(skill)) }}</span
        ><UiButton @click="skill = null">{{
          t('modules.combatJournal.analysis.allSkills')
        }}</UiButton>
      </div>
      <JournalOverview
        v-if="tab === JournalAnalysisTab.Overview"
        :segment="segment"
        :window="analysisWindow"
        :query="query"
        :formatter="formatter"
        :timeline-available="timelineAvailable"
        @select="selectActor"
        @death="selectDeath"
      />
      <template v-else-if="tab === JournalAnalysisTab.Skills">
        <div v-if="timelineAvailable" class="analysis-chart-modes">
          <button type="button" :aria-pressed="!bySkill" @click="bySkill = false">
            {{ t('modules.combatJournal.analysis.totalChart') }}</button
          ><button type="button" :aria-pressed="bySkill" @click="bySkill = true">
            {{ t('modules.combatJournal.analysis.abilityChart') }}
          </button>
        </div>
        <JournalTimeline
          v-if="timelineAvailable"
          :key="
            segment.id + ':' + String(actor) + ':' + metric + ':' + target + ':' + String(skill)
          "
          :series="bySkill ? abilitySeries : chartSeries"
          :stacked="bySkill"
          :duration="segment.spanMs ?? segment.elapsedMs"
          :formatter="formatter"
        />
        <p v-if="timelineAvailable && !full" class="analysis-warning">
          {{ t('modules.combatJournal.analysis.tracePartial') }}
        </p>
        <div class="analysis-table-tools">
          <UiSelect
            :label="t('modules.combatJournal.analysis.sort')"
            :model-value="sortMode"
            :options="
              Object.values(JournalAnalysisSort).map((value) => ({
                value,
                label: t('modules.combatJournal.analysis.sorts.' + value),
              }))
            "
            @update:model-value="sortMode = $event"
          />
        </div>
        <table class="analysis-table">
          <thead>
            <tr>
              <th>{{ t('modules.combatJournal.analysis.skill') }}</th>
              <th>{{ t('modules.combatJournal.analysis.amount') }}</th>
              <th>{{ t('modules.combatJournal.analysis.hitEvents') }}</th>
              <th>{{ t('modules.combatJournal.analysis.confirmedCasts') }}</th>
              <th>{{ t('modules.combatJournal.analysis.average') }}</th>
              <th>{{ t('modules.combatJournal.analysis.critical') }}</th>
              <th>{{ t('modules.combatJournal.analysis.periodic') }}</th>
            </tr>
          </thead>
          <tbody>
            <template v-for="entry in visible" :key="entry.row.id"
              ><tr :class="{ 'analysis-selected': skill === entry.row.id }">
                <td>
                  <button
                    type="button"
                    @click="skill = skill === entry.row.id ? null : entry.row.id"
                  >
                    <JournalSkillIcon :path="formatter.icon(Number(entry.row.id))" />
                    {{ entry.row.label }}
                  </button>
                  <div class="analysis-share">
                    <i :style="{ width: String(entry.row.share) + '%' }" />
                  </div>
                </td>
                <td>
                  {{ entry.row.value === null ? '≥ ' : '' }}{{ format(entry.amount)
                  }}<small
                    >{{ percent(entry.row.share) }} · {{ format(entry.rate)
                    }}{{ t('modules.combatJournal.analysis.perSecond') }}</small
                  >
                </td>
                <td>{{ format(entry.hits) }}</td>
                <td>{{ format(castCount(entry.row.id)) }}</td>
                <td>{{ format(entry.average) }}</td>
                <td>{{ healing ? '—' : percent(entry.critical) }}</td>
                <td>{{ format(entry.periodic) }}</td>
              </tr>
              <tr v-if="selected && skill === entry.row.id" class="analysis-detail-row">
                <td colspan="7">
                  <section class="analysis-breakdown">
                    <h3>{{ selected.row.label }}</h3>
                    <div class="analysis-drilldown">
                      <UiButton v-if="timelineAvailable" @click="tab = JournalAnalysisTab.Events">{{
                        t('modules.combatJournal.analysis.tabs.events')
                      }}</UiButton
                      ><UiButton
                        v-if="timelineAvailable"
                        @click="tab = JournalAnalysisTab.Rotation"
                        >{{ t('modules.combatJournal.analysis.tabs.rotation') }}</UiButton
                      ><UiButton @click="skill = null">{{
                        t('modules.combatJournal.analysis.allSkills')
                      }}</UiButton>
                    </div>
                    <p v-if="!healing" class="analysis-note">
                      {{ t('modules.combatJournal.analysis.directDenominator') }}
                    </p>
                    <div v-if="!healing" class="analysis-flags">
                      <div v-for="flag in flags" :key="flag.counter">
                        <span>{{ t('modules.combatJournal.counters.' + flag.counter) }}</span
                        ><b>{{ percent(flag.value) }}</b>
                      </div>
                    </div>
                    <dl>
                      <template v-if="!healing">
                        <dt>{{ t('modules.combatJournal.analysis.minimum') }}</dt>
                        <dd>{{ format(selected.row.statistics.minimum) }}</dd>
                        <dt>{{ t('modules.combatJournal.analysis.maximum') }}</dt>
                        <dd>{{ format(selected.row.statistics.maximum) }}</dd>
                      </template>
                      <template v-for="counter in otherCounters" :key="counter.counter"
                        ><dt>{{ t('modules.combatJournal.counters.' + counter.counter) }}</dt>
                        <dd>{{ format(counter.value) }}</dd></template
                      >
                    </dl>
                  </section>
                </td>
              </tr></template
            >
          </tbody>
        </table>
      </template>
      <template v-else-if="tab === JournalAnalysisTab.Targets">
        <button
          v-for="row in targetRows"
          :key="row.id"
          type="button"
          class="analysis-target"
          @click="
            target = row.id;
            tab = JournalAnalysisTab.Skills;
          "
        >
          <span>{{ row.label }}</span
          ><b>{{ formatter.value(row, false, metric) }}</b
          ><small>{{ percent(row.share) }}</small>
        </button></template
      >
      <template v-else-if="tab === JournalAnalysisTab.Timeline">
        <p v-if="!full" class="analysis-warning">
          {{ t('modules.combatJournal.analysis.tracePartial') }}
        </p>
        <JournalTimeline
          :key="
            segment.id + ':' + String(actor) + ':' + metric + ':' + target + ':' + String(skill)
          "
          :series="chartSeries"
          :duration="segment.spanMs ?? segment.elapsedMs"
          :formatter="formatter"
          @range="(start, end) => (interval = [start, end])"
        />
        <JournalIntervalPanel
          :actions="timelineEvents"
          :formatter="formatter"
          :start="interval[0]"
          :end="interval[1]"
        />
        <JournalEventsPanel :actions="timelineEvents" :segment="segment" :formatter="formatter" />
      </template>
      <template v-else-if="tab === JournalAnalysisTab.Rotation">
        <JournalCastsPanel
          :key="segment.id + ':' + String(actor) + ':' + String(raidCasts)"
          :actions="raidCasts ? groupActions : rotationActions"
          :group-actors="raidCasts"
          :actors="castActors"
          :actor-for="castActor"
          :segment="segment"
          :formatter="formatter"
          @actor="
            chooseActor($event);
            raidCasts = false;
          "
          @skill="
            skill = $event === null ? null : String($event);
            tab = JournalAnalysisTab.Events;
          "
        />
      </template>
      <JournalDeathsPanel
        v-else-if="tab === JournalAnalysisTab.Deaths"
        :segment="segment"
        :actor="actor"
        :formatter="formatter"
      />
      <JournalEventsPanel
        v-else-if="tab === JournalAnalysisTab.Events"
        :actions="eventActions"
        :segment="segment"
        :formatter="formatter"
      />
      <JournalEffectsPanel
        v-else-if="tab === JournalAnalysisTab.Effects"
        :segment="segment"
        :actor="actor"
        :formatter="formatter"
      />
      <template v-else
        ><div class="analysis-filters">
          <UiSelect
            :label="t('modules.combatJournal.analysis.reference')"
            :model-value="reference"
            :options="compareOptions"
            @update:model-value="reference = $event"
          /><UiSelect
            v-if="other"
            :label="t('modules.combatJournal.analysis.referenceActor')"
            :model-value="referenceActor"
            :options="otherActorOptions"
            @update:model-value="referenceActor = $event"
          />
        </div>
        <table v-if="other" class="analysis-table">
          <thead>
            <tr>
              <th>{{ t('modules.combatJournal.analysis.skill') }}</th>
              <th>
                {{ t('modules.combatJournal.analysis.current')
                }}{{ t('modules.combatJournal.analysis.perSecond') }}
              </th>
              <th>
                {{ t('modules.combatJournal.analysis.reference')
                }}{{ t('modules.combatJournal.analysis.perSecond') }}
              </th>
              <th>Δ {{ t('modules.combatJournal.analysis.perSecond') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="entry in compared.filter((entry) =>
                (entry.current?.row.label ?? entry.previous?.row.label ?? '')
                  .toLocaleLowerCase()
                  .includes(search.toLocaleLowerCase()),
              )"
              :key="entry.id"
            >
              <td>
                <span class="analysis-skill-name"
                  ><JournalSkillIcon :path="formatter.icon(Number(entry.id))" />{{
                    entry.current?.row.label ?? entry.previous?.row.label
                  }}</span
                >
              </td>
              <td>{{ format(entry.current?.rate ?? 0) }}</td>
              <td>{{ format(entry.previous?.rate ?? 0) }}</td>
              <td>{{ rateDelta(entry.current, entry.previous) }}</td>
            </tr>
          </tbody>
        </table></template
      >
      <p v-if="exportError" role="alert">{{ exportError }}</p>
    </div>
  </div>
</template>
<style scoped>
.analysis-history {
  display: flex;
  gap: 6px;
  padding-left: 20px;
  flex-shrink: 0;
}

.analysis-toolbar {
  display: flex;
  align-items: center;
  border-bottom: 1px solid var(--prism-border);
  flex-shrink: 0;
}

.analysis-toolbar :deep(.fight-picker) {
  flex: 1;
  min-width: 0;
  border-bottom: 0;
}

.analysis-scope {
  padding-top: 12px !important;
}
.analysis-scope :deep(.ui-switch) {
  justify-content: space-between;
  flex-direction: row-reverse;
  border: 1px solid var(--prism-border);
  border-radius: 4px;
  padding: 6px 10px;
}

.analysis-chart-modes {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
}
.analysis-chart-modes button {
  border: 1px solid var(--prism-border);
  border-radius: 4px;
  padding: 5px 10px;
  background: transparent;
  color: var(--prism-muted);
  font: inherit;
  font-size: 11px;
  cursor: pointer;
}
.analysis-chart-modes button[aria-pressed='true'] {
  color: var(--prism-text);
  background: var(--prism-selected);
  border-color: var(--prism-accent);
}
.journal-analysis {
  display: flex;
  flex-direction: column;
  min-height: 0;
  width: 100%;
  color: var(--prism-text);
  font:
    13px 'Inter Variable',
    system-ui,
    sans-serif;
}
.analysis-context {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 16px 20px;
}
.analysis-context strong {
  font-size: 18px;
}
.analysis-context small {
  display: block;
  color: var(--prism-muted);
  margin-top: 6px;
}
.analysis-summary {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 1px;
  background: var(--prism-border);
  margin: 0 20px 16px;
  border: 1px solid var(--prism-border);
}
.analysis-summary > div {
  display: flex;
  flex-direction: column;
  gap: 7px;
  padding: 12px;
  background: var(--prism-panel-background);
}
.analysis-summary span,
.analysis-flags span {
  color: var(--prism-muted);
  font-size: 12px;
}
.analysis-summary b {
  font-size: 19px;
  font-variant-numeric: tabular-nums;
}
.analysis-filters {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  padding: 0 20px 14px;
}
.analysis-tabs {
  display: flex;
  gap: 20px;
  padding: 0 20px;
  border-bottom: 1px solid var(--prism-border);
  overflow-x: auto;
  flex-shrink: 0;
}
.analysis-tabs button {
  padding: 10px 0;
  border: 0;
  border-bottom: 2px solid transparent;
  border-radius: 0;
  background: none;
  color: var(--prism-muted);
  white-space: nowrap;
  cursor: pointer;
}
.analysis-tabs button:disabled {
  opacity: 0.4;
  cursor: default;
}
.analysis-tabs button[aria-pressed='true'] {
  border-bottom-color: var(--prism-accent);
  color: var(--prism-text);
}
.analysis-scroll {
  overflow: auto;
  min-height: 0;
  padding: 16px 20px;
}
.analysis-scroll-rotation {
  display: flex;
  flex: 1;
  flex-direction: column;
  overflow: hidden;
}
.analysis-scroll > .analysis-context {
  padding: 0 0 14px;
}
.analysis-scroll > .analysis-filters {
  padding: 0 0 14px;
}
.analysis-scroll > .analysis-summary {
  margin: 0 0 16px;
}
.analysis-scroll-rotation > :not(.journal-casts-panel) {
  flex-shrink: 0;
}
.analysis-table-tools {
  margin-top: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  margin-bottom: 12px;
  color: var(--prism-muted);
  font-size: 11px;
}
.analysis-table-tools :deep(.ui-select) {
  flex: 0 0 180px;
  width: 180px;
  min-width: 0;
}
.analysis-table {
  width: 100%;
  border-collapse: collapse;
  font-variant-numeric: tabular-nums;
}
.analysis-table th {
  font-size: 11px;
  color: var(--prism-muted);
  font-weight: 500;
  text-align: right;
  padding: 8px;
  border-bottom: 1px solid var(--prism-border);
}
.analysis-table th:first-child,
.analysis-table td:first-child {
  text-align: left;
  width: 30%;
  white-space: normal;
}
.analysis-table td {
  white-space: nowrap;
  text-align: right;
  padding: 12px 8px;
  border-bottom: 1px solid var(--prism-border);
  vertical-align: top;
}
.analysis-table td small {
  display: block;
  color: var(--prism-muted);
  font-size: 11px;
  margin-top: 5px;
}
.analysis-table button {
  display: flex;
  align-items: center;
  gap: 8px;
  background: none;
  border: 0;
  color: inherit;
  text-align: left;
  padding: 0;
  cursor: pointer;
  font: inherit;
}
.analysis-share {
  height: 3px;
  background: var(--prism-field);
  margin-top: 7px;
}
.analysis-share i {
  display: block;
  background: var(--prism-accent);
  height: 100%;
}
.analysis-selected {
  background: var(--prism-selected);
}
.analysis-breakdown {
  text-align: left;
  white-space: normal;
  padding: 8px 12px;
}
.analysis-breakdown h3 {
  font-size: 14px;
}
.analysis-flags {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 12px;
  padding: 12px 0;
}
.analysis-flags > div {
  display: flex;
  flex-direction: column;
  gap: 7px;
}
.analysis-breakdown dl {
  display: grid;
  grid-template-columns: 1fr auto 1fr auto;
  gap: 8px;
}
.analysis-breakdown dd {
  margin: 0 20px 0 0;
  font-variant-numeric: tabular-nums;
}
.analysis-note {
  font-size: 12px;
  line-height: 1.5;
  color: var(--prism-muted);
  margin: 0 0 12px;
}
.analysis-warning {
  color: var(--prism-accent);
  font-size: 12px;
  line-height: 1.5;
}
.analysis-target {
  display: flex;
  width: 100%;
  justify-content: space-between;
  gap: 15px;
  text-align: left;
  padding: 12px;
  background: var(--prism-field);
  color: inherit;
  border: 0;
  border-bottom: 1px solid var(--prism-border);
  cursor: pointer;
}
.analysis-target span {
  flex: 1;
}
.analysis-target small {
  color: var(--prism-muted);
}
@media (max-width: 650px) {
  .analysis-summary {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
  .analysis-flags {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
  .analysis-filters {
    grid-template-columns: 1fr;
  }
  .analysis-table th,
  .analysis-table td {
    padding: 9px 4px;
    font-size: 11px;
  }
}
.analysis-active-filter,
.analysis-drilldown {
  display: flex;
  align-items: center;
  gap: 12px;
}
.analysis-active-filter {
  padding: 8px 20px;
  background: var(--prism-selected);
  font-size: 12px;
}
.analysis-skill-name {
  display: flex;
  gap: 8px;
  align-items: center;
}
.analysis-scroll > .journal-events-panel {
  margin-top: 20px;
}
.analysis-table tbody tr:hover {
  background: var(--prism-field);
}
.analysis-table th {
  position: sticky;
  top: 0;
  background: var(--prism-panel-background);
  z-index: 1;
}
</style>
