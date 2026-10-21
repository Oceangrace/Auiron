<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalFormatter } from '../runtime/journal-formatter';
import { JournalUptime } from '../runtime/journal-uptime';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import JournalSkillIcon from './journal-skill-icon.vue';
import { EffectKind } from '../../../core/catalog/effect-kind';
const props = defineProps<{
  segment: JournalSegment;
  actor: number;
  formatter: JournalFormatter;
}>();

const { t } = useI18n();

const selected = ref(String(props.actor));

const search = ref('');

const kind = ref<EffectKind | 'all'>('all');

const kinds = computed(() => [
  { value: 'all' as const, label: t('modules.combatJournal.analysis.allEffects') },
  { value: EffectKind.Buff, label: t('modules.combatJournal.analysis.buffs') },
  { value: EffectKind.Debuff, label: t('modules.combatJournal.analysis.debuffs') },
  { value: EffectKind.Passive, label: t('modules.combatJournal.analysis.passive') },
]);

const uptime = new JournalUptime();

const matchesKind = (effect: number | null): boolean => {
  const value = props.formatter.effectKind(effect);

  if (kind.value === EffectKind.Passive) {
    return value === EffectKind.Passive || value === EffectKind.ItemPassive;
  }

  return kind.value === 'all' || value === kind.value;
};

const units = computed(() =>
  [...new Set([props.actor, ...(props.segment.effects ?? []).map((effect) => effect.unitId)])].map(
    (id) => ({ value: String(id), label: props.formatter.unit(id, props.segment) }),
  ),
);

const rows = computed(() =>
  uptime
    .rows(props.segment, Number(selected.value))
    .filter((row) => props.formatter.effectVisible(row.effectId))
    .filter((row) =>
      props.formatter
        .effect(row.effectId, row.skillId)
        .toLocaleLowerCase()
        .includes(search.value.toLocaleLowerCase()),
    )
    .filter((row) => matchesKind(row.effectId)),
);

watch(
  () => props.actor,
  () => {
    selected.value = String(props.actor);
  },
);
</script>
<template>
  <div class="journal-effects-panel">
    <div class="effect-filters">
      <UiSelect
        :label="t('modules.combatJournal.analysis.effectSubject')"
        :model-value="selected"
        :options="units"
        @update:model-value="selected = $event"
      />
      <UiSelect
        :label="t('modules.combatJournal.analysis.allEffects')"
        :model-value="kind"
        :options="kinds"
        @update:model-value="kind = $event"
      />
      <input
        v-model="search"
        class="ui-input"
        :aria-label="t('modules.combatJournal.analysis.effectSearch')"
        :placeholder="t('modules.combatJournal.analysis.effectSearch')"
      />
    </div>
    <p v-if="segment.tracePartial">{{ t('modules.combatJournal.analysis.tracePartial') }}</p>
    <div v-for="row in rows" :key="row.key" class="effect-row">
      <div>
        <strong
          ><JournalSkillIcon :path="formatter.icon(row.skillId, row.effectId)" />{{
            formatter.effect(row.effectId, row.skillId)
          }}</strong
        ><span
          >{{ formatter.time(row.milliseconds) }} ·
          {{
            formatter.number(
              (row.milliseconds * 100) / Math.max(1, segment.spanMs ?? segment.elapsedMs),
              false,
            )
          }}%</span
        >
      </div>
      <div class="effect-track">
        <i
          v-for="(interval, index) in row.intervals"
          :key="index"
          :title="formatter.time(interval[0]) + ' – ' + formatter.time(interval[1])"
          :style="{
            left:
              String((interval[0] * 100) / Math.max(1, segment.spanMs ?? segment.elapsedMs)) + '%',
            width:
              String(
                ((interval[1] - interval[0]) * 100) /
                  Math.max(1, segment.spanMs ?? segment.elapsedMs),
              ) + '%',
          }"
        />
      </div>
    </div>
    <p v-if="rows.length === 0">{{ t('modules.combatJournal.analysis.noEffects') }}</p>
  </div>
</template>
<style scoped>
.journal-effects-panel {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.journal-effects-panel p {
  font-size: 12px;
  line-height: 1.5;
  color: var(--prism-muted);
  margin: 0;
}
.effect-row > div:first-child {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  font-size: 12px;
}
.effect-row strong {
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 8px;
}
.effect-row > div:first-child {
  align-items: center;
}
.effect-filters {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 12px;
}
.effect-row {
  display: grid;
  grid-template-columns: minmax(260px, 40%) 1fr;
  gap: 20px;
  align-items: center;
  padding: 6px 0;
  border-bottom: 1px solid var(--prism-border);
}
@media (max-width: 700px) {
  .effect-row,
  .effect-filters {
    grid-template-columns: 1fr;
  }
}
.effect-row span {
  font-variant-numeric: tabular-nums;
  color: var(--prism-muted);
}
.effect-track {
  position: relative;
  height: 15px;
  background: var(--prism-field);
  margin-top: 8px;
  border-left: 1px solid var(--prism-border);
  border-right: 1px solid var(--prism-border);
}
.effect-track i {
  position: absolute;
  top: 0;
  height: 100%;
  background: var(--prism-accent);
  opacity: 0.65;
}
</style>
