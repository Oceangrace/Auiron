<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalFormatter } from '../runtime/journal-formatter';
import { JournalPeriod } from '../model/journal-period';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';

const props = defineProps<{
  segment: JournalSegment;
  segments: readonly JournalSegment[];
  formatter: JournalFormatter;
}>();

const emit = defineEmits<{ select: [id: string] }>();

const { t } = useI18n();

const location = ref('all');

const fights = computed(() =>
  [...new Map([props.segment, ...props.segments].map((fight) => [fight.id, fight])).values()].sort(
    (a, b) => b.startedAtUs - a.startedAtUs,
  ),
);

const locations = computed(() => [
  { value: 'all', label: t('modules.combatJournal.analysis.allLocations') },
  ...[...new Set(fights.value.map((fight) => fight.mapId))].map((id) => ({
    value: String(id),
    label:
      props.formatter.zone({ ...props.segment, mapId: id }) ??
      t('modules.combatJournal.analysis.unknownLocation'),
  })),
]);

const options = computed(() =>
  fights.value
    .filter((fight) => location.value === 'all' || String(fight.mapId) === location.value)
    .map((fight) => ({
      value: fight.id,
      label: [
        fight.period === JournalPeriod.Run
          ? t('modules.combatJournal.periods.run') + ' · ' + (props.formatter.zone(fight) ?? '—')
          : props.formatter.npc(fight.npcId),
        props.formatter.time(fight.elapsedMs),
        new Date(fight.startedAtUs / 1000).toLocaleString(),
      ].join(' · '),
    })),
);

const selectLocation = (value: string): void => {
  location.value = value;

  if (options.value.some((option) => option.value === props.segment.id) === false) {
    const first = options.value[0];

    if (first !== undefined) {
      emit('select', first.value);
    }
  }
};
</script>
<template>
  <div class="fight-picker">
    <UiSelect
      :label="t('modules.combatJournal.analysis.location')"
      :model-value="location"
      :options="locations"
      @update:model-value="selectLocation"
    />
    <UiSelect
      :label="t('modules.combatJournal.analysis.chooseFight')"
      :model-value="segment.id"
      :options="options"
      @update:model-value="emit('select', $event)"
    />
  </div>
</template>
<style scoped>
.fight-picker {
  display: grid;
  grid-template-columns: minmax(140px, 1fr) 2fr;
  gap: 12px;
  padding: 14px 20px;
  border-bottom: 1px solid var(--prism-border);
}
@media (max-width: 650px) {
  .fight-picker {
    grid-template-columns: 1fr;
  }
}
</style>
