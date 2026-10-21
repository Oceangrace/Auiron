<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { CombatJournalModule } from '../combat-journal-module';
import type { JournalPreferences } from '../model/journal-preferences';
import type { JournalWindow } from '../model/journal-window';
import type { JournalAppearance } from '../model/journal-appearance';
import { JournalSettingsSection } from '../model/journal-settings-section';
import { JournalMetric } from '../model/journal-metric';
import { JournalScope } from '../model/journal-scope';
import { JournalPeriod } from '../model/journal-period';
import JournalAppearanceEditor from './journal-appearance-editor.vue';
import JournalHeaderButtons from './journal-header-buttons.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiSlider from '../../../renderer/components/ui/ui-slider.vue';
import UiColor from '../../../renderer/components/ui/ui-color.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
const props = defineProps<{
  journal: CombatJournalModule;
  preferences: JournalPreferences;
  window: JournalWindow;
}>();

const { t } = useI18n();

const section = ref(JournalSettingsSection.Window);

const appearance = computed(() => props.preferences.appearance);

const profile = computed(() => section.value !== JournalSettingsSection.Window);

const options = <T extends string>(
  values: Readonly<Record<string, T>>,
  prefix: string,
): { value: T; label: string }[] =>
  Object.values(values).map((value) => ({ value, label: t(prefix + value) }));

const toggles = [
  { key: 'showRank', label: 'showRank' },
  { key: 'showPercent', label: 'showPercent' },
  { key: 'classColors', label: 'classColors' },
  { key: 'bars', label: 'bars' },
  { key: 'compactNumbers', label: 'compact' },
  { key: 'showTotal', label: 'showTotal' },
] as const;

const windows = computed(() => {
  const counts = new Map<string, number>();

  return props.preferences.windows.map((entry) => {
    const label = [
      t('modules.combatJournal.metrics.' + entry.metric),
      t('modules.combatJournal.scopes.' + entry.scope),
      t('modules.combatJournal.periods.' + entry.period),
    ].join(' · ');

    const count = (counts.get(label) ?? 0) + 1;

    counts.set(label, count);

    return { value: entry.id, label: count === 1 ? label : `${label} (${String(count)})` };
  });
});

const update = (patch: Partial<JournalWindow>): void => {
  props.journal.update({ ...props.window, ...patch });
};

const patch = (value: Partial<JournalAppearance>): void => {
  props.journal.updateAppearance(value);
};
</script>
<template>
  <div class="journal-settings-shell">
    <div class="journal-settings-target">
      <UiField :label="t('modules.combatJournal.editingWindow')"
        ><UiSelect
          :label="t('modules.combatJournal.editingWindow')"
          :model-value="window.id"
          :options="windows"
          @update:model-value="journal.openSettings($event)"
      /></UiField>
      <UiButton :disabled="preferences.windows.length >= 8" @click="journal.duplicate(window.id)">{{
        t('modules.combatJournal.duplicate')
      }}</UiButton
      ><UiButton :disabled="preferences.windows.length <= 1" @click="journal.remove(window.id)">{{
        t('modules.combatJournal.remove')
      }}</UiButton>
    </div>
    <nav class="journal-settings-tabs">
      <button
        v-for="item in Object.values(JournalSettingsSection)"
        :key="item"
        type="button"
        :aria-pressed="section === item"
        @click="section = item"
      >
        {{ t('modules.combatJournal.sections.' + item) }}
      </button>
    </nav>
    <div class="journal-settings">
      <template v-if="section === JournalSettingsSection.Window">
        <UiField :label="t('modules.combatJournal.metric')"
          ><UiSelect
            :label="t('modules.combatJournal.metric')"
            :model-value="window.metric"
            :options="options(JournalMetric, 'modules.combatJournal.metrics.')"
            @update:model-value="update({ metric: $event as JournalMetric })"
        /></UiField>
        <UiField :label="t('modules.combatJournal.scope')"
          ><UiSelect
            :label="t('modules.combatJournal.scope')"
            :model-value="window.scope"
            :options="options(JournalScope, 'modules.combatJournal.scopes.')"
            @update:model-value="update({ scope: $event as JournalScope })"
        /></UiField>
        <UiField :label="t('modules.combatJournal.period')"
          ><UiSelect
            :label="t('modules.combatJournal.period')"
            :model-value="window.period"
            :options="options(JournalPeriod, 'modules.combatJournal.periods.')"
            @update:model-value="update({ period: $event as JournalPeriod })"
        /></UiField>
        <UiField :label="t('modules.combatJournal.rows')"
          ><UiNumber
            :label="t('modules.combatJournal.rows')"
            :model-value="window.limit"
            :min="1"
            :max="40"
            @update:model-value="update({ limit: $event })"
        /></UiField>
        <UiSwitch
          :model-value="window.mergeSummons"
          :label="t('modules.combatJournal.summons')"
          @update:model-value="update({ mergeSummons: $event })"
        />
        <UiSwitch
          :model-value="window.locked"
          :label="t('modules.combatJournal.lock')"
          @update:model-value="update({ locked: $event })"
        />
        <UiField class="journal-settings-wide" :label="t('modules.combatJournal.history')">
          <div class="journal-settings-actions">
            <UiButton @click="journal.history.resetRun()">{{
              t('modules.combatJournal.newRun')
            }}</UiButton>
            <UiButton @click="journal.clearHistory()">{{
              t('modules.combatJournal.clear')
            }}</UiButton>
          </div>
        </UiField>
      </template>
      <template v-if="section === JournalSettingsSection.Display">
        <UiField :label="t('modules.combatJournal.opacity')"
          ><UiSlider
            :model-value="Math.round(appearance.opacity * 100)"
            :label="t('modules.combatJournal.opacity')"
            :min="0"
            :max="100"
            @update:model-value="patch({ opacity: $event / 100 })"
        /></UiField>
        <UiField :label="t('modules.combatJournal.color')"
          ><UiColor
            :model-value="appearance.accent"
            :label="t('modules.combatJournal.color')"
            @update:model-value="patch({ accent: $event.slice(0, 7) })"
        /></UiField>
      </template>
      <template v-if="section === JournalSettingsSection.Rows">
        <UiField :label="t('modules.combatJournal.font')"
          ><UiNumber
            :label="t('modules.combatJournal.font')"
            :model-value="appearance.fontSize"
            :min="10"
            :max="32"
            @update:model-value="patch({ fontSize: $event })"
        /></UiField>
        <UiField :label="t('modules.combatJournal.rowHeight')"
          ><UiNumber
            :label="t('modules.combatJournal.rowHeight')"
            :model-value="appearance.rowHeight"
            :min="14"
            :max="64"
            @update:model-value="patch({ rowHeight: $event })"
        /></UiField>
        <UiSwitch
          v-for="toggle in toggles"
          :key="toggle.key"
          :model-value="appearance[toggle.key]"
          :label="t('modules.combatJournal.' + toggle.label)"
          @update:model-value="patch({ [toggle.key]: $event })"
        />
      </template>
      <template v-if="section === JournalSettingsSection.Header">
        <UiField :label="t('modules.combatJournal.headerButtonSize')"
          ><UiNumber
            :label="t('modules.combatJournal.headerButtonSize')"
            :model-value="appearance.headerButtonSize"
            :min="12"
            :max="32"
            @update:model-value="patch({ headerButtonSize: $event })"
        /></UiField>
        <UiSwitch
          :model-value="appearance.headerButtonsOnHover"
          :label="t('modules.combatJournal.headerButtonsOnHover')"
          @update:model-value="patch({ headerButtonsOnHover: $event })"
        />
        <UiField class="journal-settings-wide" :label="t('modules.combatJournal.headerButtonList')">
          <JournalHeaderButtons
            :buttons="appearance.headerButtons"
            @change="patch({ headerButtons: $event })"
          />
        </UiField>
      </template>
      <JournalAppearanceEditor
        v-if="profile"
        class="journal-settings-wide"
        :appearance="appearance"
        :section="section"
        @patch="patch"
      />
    </div>
  </div>
</template>
<style scoped>
.journal-settings-target {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto auto;
  align-items: end;
  gap: 12px;
  padding: 16px 16px 8px;
}
.journal-settings-tabs {
  display: flex;
  gap: 16px;
  padding: 0 16px;
  border-bottom: 1px solid var(--prism-border);
}
.journal-settings-tabs button {
  border-radius: 0;
  border: 0;
  border-bottom: 2px solid transparent;
  padding: 14px 0;
  background: transparent;
  color: var(--prism-muted);
  cursor: pointer;
}
.journal-settings-tabs button[aria-pressed='true'] {
  color: var(--prism-text);
  border-color: var(--prism-accent);
}
.journal-settings {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
  padding: 16px;
}
.journal-settings-wide {
  grid-column: 1/-1;
}
.journal-settings > :deep(.ui-switch) {
  grid-column: 1 / -1;
  flex-direction: row-reverse;
  justify-content: space-between;
  min-height: 40px;
  padding: 8px 12px;
  border: 1px solid var(--prism-border);
  border-radius: 5px;
  background: var(--prism-field);
}
.journal-settings > small {
  grid-column: 1 / -1;
  color: var(--prism-muted);
}
.journal-settings-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}
</style>
