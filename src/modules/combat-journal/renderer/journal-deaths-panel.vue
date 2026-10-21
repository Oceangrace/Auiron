<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalFormatter } from '../runtime/journal-formatter';
import { JournalActionKind } from '../model/journal-action-kind';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import JournalSkillIcon from './journal-skill-icon.vue';
const props = defineProps<{
  segment: JournalSegment;
  actor: number;
  formatter: JournalFormatter;
}>();

const { t } = useI18n();

const deaths = computed(() =>
  (props.segment.actions ?? []).filter(
    (action) => action.kind === JournalActionKind.Death && action.sourceId === props.actor,
  ),
);

const selected = ref('');

const choices = computed(() =>
  deaths.value.map((action) => ({
    value: String(action.sequence),
    label: props.formatter.time((action.atUs - props.segment.startedAtUs) / 1000),
  })),
);

const death = computed(
  () =>
    deaths.value.find((action) => String(action.sequence) === selected.value) ??
    deaths.value.at(-1),
);

const events = computed(() => {
  const end = death.value?.atUs ?? 0;

  const sequence = death.value?.sequence ?? 0;

  return (props.segment.actions ?? [])
    .filter(
      (action) =>
        action.recipientId === props.actor &&
        action.atUs <= end &&
        (action.atUs !== end || action.sequence <= sequence) &&
        action.atUs >= end - 10_000_000,
    )
    .filter(
      (action) =>
        action.kind === JournalActionKind.Damage || action.kind === JournalActionKind.Healing,
    )
    .toSorted((a, b) => (a.atUs === b.atUs ? a.sequence - b.sequence : a.atUs - b.atUs));
});

const lastDamage = computed(() =>
  events.value.filter((action) => action.kind === JournalActionKind.Damage).at(-1),
);

const totals = computed(() =>
  [JournalActionKind.Damage, JournalActionKind.Healing].map((kind) => ({
    kind,
    value: events.value
      .filter((action) => action.kind === kind)
      .reduce((sum, action) => sum + action.value, 0),
  })),
);

watch(
  () => props.actor,
  () => {
    selected.value = '';
  },
);
</script>
<template>
  <div class="journal-deaths-panel">
    <p v-if="segment.tracePartial">{{ t('modules.combatJournal.analysis.tracePartial') }}</p>
    <UiSelect
      v-if="deaths.length !== 0"
      :label="t('modules.combatJournal.analysis.deathMoment')"
      :model-value="String(death?.sequence ?? '')"
      :options="choices"
      @update:model-value="selected = $event"
    />
    <p v-else>{{ t('modules.combatJournal.analysis.noDeaths') }}</p>
    <div v-if="death" class="death-totals">
      <span>{{ t('modules.combatJournal.analysis.lastTenSeconds') }}</span
      ><strong v-for="total in totals" :key="total.kind" :class="total.kind"
        >{{ t('modules.combatJournal.analysis.kinds.' + total.kind) }}:
        {{ formatter.number(total.value, false) }}</strong
      >
    </div>
    <p v-if="lastDamage" class="death-last-hit">
      {{ t('modules.combatJournal.analysis.lastRecordedHit') }}:
      <JournalSkillIcon :path="formatter.icon(lastDamage.skillId)" />
      <b>{{ formatter.skill(lastDamage.skillId) }}</b> ·
      {{ formatter.unit(lastDamage.sourceId, segment) }} ·
      {{ formatter.number(lastDamage.value, false) }}
    </p>
    <table v-if="death">
      <thead>
        <tr>
          <th>{{ t('modules.combatJournal.analysis.beforeDeath') }}</th>
          <th>{{ t('modules.combatJournal.analysis.source') }}</th>
          <th>{{ t('modules.combatJournal.analysis.skill') }}</th>
          <th>{{ t('modules.combatJournal.analysis.amountValue') }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="action in events" :key="action.sequence">
          <td>
            {{ formatter.number((action.atUs - death.atUs) / 1_000_000, false)
            }}{{ t('modules.combatJournal.analysis.secondsShort') }}
          </td>
          <td>{{ formatter.unit(action.sourceId, segment) }}</td>
          <td>
            <span class="death-skill"
              ><JournalSkillIcon :path="formatter.icon(action.skillId)" />{{
                formatter.skill(action.skillId)
              }}</span
            >
          </td>
          <td :class="action.kind">
            {{ action.kind === JournalActionKind.Healing ? '+' : '−'
            }}{{ formatter.number(action.value, false) }}
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
<style scoped>
.death-last-hit {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}
summary {
  cursor: pointer;
  padding: 10px 0;
  font-size: 12px;
}
.death-totals {
  display: flex;
  gap: 24px;
  align-items: center;
  padding: 16px;
  border: 1px solid var(--prism-border);
  border-radius: 5px;
  font-size: 12px;
  flex-wrap: wrap;
}
.death-skill {
  display: flex;
  align-items: center;
  gap: 8px;
}
.journal-deaths-panel {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.journal-deaths-panel p {
  font-size: 12px;
  line-height: 1.5;
  color: var(--prism-muted);
  margin: 0;
}
.journal-deaths-panel table {
  border-collapse: collapse;
  width: 100%;
  font-size: 12px;
}
.journal-deaths-panel th {
  text-align: left;
  color: var(--prism-muted);
  font-weight: 500;
}
.journal-deaths-panel th,
.journal-deaths-panel td {
  padding: 10px 5px;
  border-bottom: 1px solid var(--prism-border);
}
.journal-deaths-panel td:last-child {
  text-align: right;
  font-variant-numeric: tabular-nums;
}
.damage {
  color: #d97878;
}
.healing {
  color: #70bc95;
}
</style>
