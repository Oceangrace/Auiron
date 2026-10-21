<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalAction } from '../model/journal-action';
import type { JournalFormatter } from '../runtime/journal-formatter';
import { JournalInterval } from '../runtime/journal-interval';
import JournalSkillIcon from './journal-skill-icon.vue';

const props = defineProps<{
  actions: readonly JournalAction[];
  formatter: JournalFormatter;
  start: number;
  end: number;
}>();

const { t } = useI18n();

const interval = new JournalInterval();

const rows = computed(() => interval.rows(props.actions));

const rate = (value: number): string =>
  props.end > props.start
    ? props.formatter.number((value * 1000) / (props.end - props.start), false)
    : '—';
</script>
<template>
  <details class="interval-panel" open>
    <summary>
      {{ t('modules.combatJournal.analysis.intervalTotals') }} · {{ formatter.time(start) }} –
      {{ formatter.time(end) }}
    </summary>
    <p>{{ t('modules.combatJournal.analysis.intervalHint') }}</p>
    <table>
      <thead>
        <tr>
          <th>{{ t('modules.combatJournal.analysis.skill') }}</th>
          <th>{{ t('modules.combatJournal.metrics.damage') }}</th>
          <th>DPS</th>
          <th>{{ t('modules.combatJournal.metrics.healing') }}</th>
          <th>HPS</th>
          <th>{{ t('modules.combatJournal.analysis.hitEvents') }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in rows" :key="String(row.skillId)">
          <td>
            <span
              ><JournalSkillIcon :path="formatter.icon(row.skillId)" />{{
                formatter.skill(row.skillId)
              }}</span
            >
          </td>
          <td>{{ formatter.number(row.damage, false) }}</td>
          <td>{{ rate(row.damage) }}</td>
          <td>{{ formatter.number(row.healing, false) }}</td>
          <td>{{ rate(row.healing) }}</td>
          <td>{{ row.hits }}</td>
        </tr>
      </tbody>
    </table>
  </details>
</template>
<style scoped>
.interval-panel {
  margin: 16px 0;
  font-size: 12px;
}
summary {
  cursor: pointer;
  padding: 10px 0;
  font-weight: 600;
}
p {
  color: var(--prism-muted);
}
table {
  width: 100%;
  border-collapse: collapse;
  font-variant-numeric: tabular-nums;
}
th,
td {
  text-align: right;
  padding: 8px;
  border-bottom: 1px solid var(--prism-border);
}
th {
  font-size: 11px;
  color: var(--prism-muted);
  font-weight: 500;
}
th:first-child,
td:first-child {
  text-align: left;
}
td span {
  display: flex;
  gap: 8px;
  align-items: center;
}
</style>
