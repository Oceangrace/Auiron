<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalAction } from '../model/journal-action';
import type { JournalSegment } from '../model/journal-segment';
import type { JournalFormatter } from '../runtime/journal-formatter';
import { JournalActionKind } from '../model/journal-action-kind';
import JournalSkillIcon from './journal-skill-icon.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';

const props = defineProps<{
  actions: readonly JournalAction[];
  segment: JournalSegment;
  formatter: JournalFormatter;
}>();

const { t } = useI18n();

const kind = ref<JournalActionKind | 'all'>('all');

const search = ref('');

const page = ref(0);

const options = computed(() => [
  { value: 'all' as const, label: t('modules.combatJournal.analysis.allEvents') },
  ...Object.values(JournalActionKind).map((value) => ({
    value,
    label: t('modules.combatJournal.analysis.kinds.' + value),
  })),
]);

const filtered = computed(() =>
  props.actions
    .filter(
      (action) =>
        (kind.value === 'all' || action.kind === kind.value) &&
        [
          props.formatter.skill(action.skillId),
          props.formatter.unit(action.sourceId, props.segment),
          action.recipientId === null
            ? ''
            : props.formatter.unit(action.recipientId, props.segment),
        ]
          .join(' ')
          .toLocaleLowerCase()
          .includes(search.value.toLocaleLowerCase()),
    )
    .toSorted((a, b) => (a.atUs === b.atUs ? a.sequence - b.sequence : a.atUs - b.atUs)),
);

const pages = computed(() => Math.max(1, Math.ceil(filtered.value.length / 50)));

const visible = computed(() => filtered.value.slice(page.value * 50, (page.value + 1) * 50));

watch([kind, search, () => props.segment.id], () => {
  page.value = 0;
});
watch(pages, () => {
  page.value = Math.min(page.value, pages.value - 1);
});
</script>
<template>
  <div class="journal-events-panel">
    <p v-if="segment.tracePartial">{{ t('modules.combatJournal.analysis.tracePartial') }}</p>
    <div class="event-tools">
      <UiSelect
        :label="t('modules.combatJournal.analysis.allEvents')"
        :model-value="kind"
        :options="options"
        @update:model-value="kind = $event"
      /><input
        v-model="search"
        class="ui-input"
        :placeholder="t('modules.combatJournal.analysis.searchEvents')"
        :aria-label="t('modules.combatJournal.analysis.searchEvents')"
      />
    </div>
    <table>
      <thead>
        <tr>
          <th>{{ t('modules.combatJournal.analysis.time') }}</th>
          <th>{{ t('modules.combatJournal.analysis.source') }}</th>
          <th>{{ t('modules.combatJournal.analysis.skill') }}</th>
          <th>{{ t('modules.combatJournal.analysis.target') }}</th>
          <th>{{ t('modules.combatJournal.analysis.amountValue') }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="action in visible" :key="action.sequence">
          <td class="event-time">
            {{ formatter.time((action.atUs - segment.startedAtUs) / 1000)
            }}<small>{{ t('modules.combatJournal.analysis.kinds.' + action.kind) }}</small>
          </td>
          <td>{{ formatter.unit(action.sourceId, segment) }}</td>
          <td>
            <span class="event-skill"
              ><JournalSkillIcon :path="formatter.icon(action.skillId)" />{{
                formatter.skill(action.skillId)
              }}</span
            >
          </td>
          <td>
            {{ action.recipientId === null ? '—' : formatter.unit(action.recipientId, segment) }}
          </td>
          <td :class="action.kind">
            {{
              [JournalActionKind.Damage, JournalActionKind.Healing].includes(action.kind)
                ? formatter.number(action.value, false)
                : '—'
            }}
          </td>
        </tr>
      </tbody>
    </table>
    <p v-if="visible.length === 0">{{ t('modules.combatJournal.analysis.noEvents') }}</p>
    <footer>
      <span
        >{{ formatter.number(filtered.length, false) }}
        {{ t('modules.combatJournal.analysis.tabs.events') }}</span
      >
      <div>
        <UiButton :disabled="page === 0" @click="page--">{{
          t('modules.combatJournal.analysis.previous')
        }}</UiButton
        ><span>{{ page + 1 }} / {{ pages }}</span
        ><UiButton :disabled="page + 1 >= pages" @click="page++">{{
          t('modules.combatJournal.analysis.next')
        }}</UiButton>
      </div>
    </footer>
  </div>
</template>
<style scoped>
.event-tools {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-bottom: 12px;
}
table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
  font-variant-numeric: tabular-nums;
}
th {
  color: var(--prism-muted);
  font-size: 11px;
  font-weight: 500;
  text-align: left;
}
td,
th {
  padding: 8px;
  border-bottom: 1px solid var(--prism-border);
}
td:last-child,
th:last-child {
  text-align: right;
}
.event-skill {
  display: flex;
  align-items: center;
  gap: 8px;
}
.event-time {
  white-space: nowrap;
}
small {
  display: block;
  font-size: 10px;
  color: var(--prism-muted);
  margin-top: 3px;
}
.damage {
  color: #dc8888;
}
.healing {
  color: #7bc99f;
}
footer,
footer > div {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
footer {
  padding-top: 14px;
  font-size: 11px;
  color: var(--prism-muted);
}
</style>
