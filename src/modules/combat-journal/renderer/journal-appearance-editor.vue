<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { JournalAppearance } from '../model/journal-appearance';
import { JournalSettingsSection } from '../model/journal-settings-section';
import { JournalBorderStyle } from '../model/journal-border-style';
import { JournalFont } from '../model/journal-font';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiColor from '../../../renderer/components/ui/ui-color.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
const props = defineProps<{ appearance: JournalAppearance; section: JournalSettingsSection }>();

const emit = defineEmits<{ patch: [value: Partial<JournalAppearance>] }>();

const { t } = useI18n();

const colors = [
  { key: 'backgroundColor', section: JournalSettingsSection.Display },
  { key: 'borderColor', section: JournalSettingsSection.Display },
  { key: 'headerColor', section: JournalSettingsSection.Header },
  { key: 'headerTextColor', section: JournalSettingsSection.Header },
  { key: 'rowBackgroundColor', section: JournalSettingsSection.Rows },
  { key: 'textColor', section: JournalSettingsSection.Rows },
  { key: 'valueColor', section: JournalSettingsSection.Rows },
  { key: 'totalColor', section: JournalSettingsSection.Rows },
] as const;

const numbers = [
  { key: 'borderWidth', min: 0, max: 4, section: JournalSettingsSection.Display },
  { key: 'padding', min: 0, max: 24, section: JournalSettingsSection.Display },
  { key: 'headerOpacity', min: 0, max: 100, section: JournalSettingsSection.Header },
  { key: 'headerHeight', min: 20, max: 48, section: JournalSettingsSection.Header },
  { key: 'headerFontSize', min: 8, max: 24, section: JournalSettingsSection.Header },
  { key: 'rowBackgroundOpacity', min: 0, max: 100, section: JournalSettingsSection.Rows },
  { key: 'rowGap', min: 0, max: 12, section: JournalSettingsSection.Rows },
  { key: 'rowPadding', min: 0, max: 16, section: JournalSettingsSection.Rows },
  { key: 'rowRadius', min: 0, max: 12, section: JournalSettingsSection.Rows },
  { key: 'barOpacity', min: 0, max: 100, section: JournalSettingsSection.Rows },
  { key: 'decimals', min: 0, max: 3, section: JournalSettingsSection.Rows },
  { key: 'animationMs', min: 0, max: 2000, section: JournalSettingsSection.Rows },
] as const;

const colorFields = computed(() => colors.filter((field) => field.section === props.section));

const numberFields = computed(() => numbers.filter((field) => field.section === props.section));

const borders = computed(() =>
  Object.values(JournalBorderStyle).map((value) => ({
    value,
    label: t('modules.combatJournal.borderStyles.' + value),
  })),
);

const fonts = computed(() =>
  Object.values(JournalFont).map((value) => ({
    value,
    label: t('modules.combatJournal.fonts.' + value),
  })),
);
</script>
<template>
  <div class="journal-appearance-editor">
    <UiSwitch
      v-if="section === JournalSettingsSection.Display"
      class="journal-custom-colors"
      :model-value="appearance.customColors"
      :label="t('modules.combatJournal.customColors')"
      @update:model-value="emit('patch', { customColors: $event })"
    />
    <UiField
      v-for="field in appearance.customColors ? colorFields : []"
      :key="field.key"
      :label="t('modules.combatJournal.design.' + field.key)"
      ><UiColor
        :model-value="appearance[field.key]"
        :label="t('modules.combatJournal.design.' + field.key)"
        @update:model-value="emit('patch', { [field.key]: $event.slice(0, 7) })"
    /></UiField>
    <UiField
      v-for="field in numberFields"
      :key="field.key"
      :label="t('modules.combatJournal.design.' + field.key)"
      ><UiNumber
        :model-value="appearance[field.key]"
        :label="t('modules.combatJournal.design.' + field.key)"
        :min="field.min"
        :max="field.max"
        @update:model-value="emit('patch', { [field.key]: $event })"
    /></UiField>
    <UiField
      v-if="section === JournalSettingsSection.Display"
      :label="t('modules.combatJournal.design.borderStyle')"
      ><UiSelect
        :model-value="appearance.borderStyle"
        :options="borders"
        :label="t('modules.combatJournal.design.borderStyle')"
        @update:model-value="emit('patch', { borderStyle: $event })"
    /></UiField>
    <UiSwitch
      v-if="section === JournalSettingsSection.Header"
      :model-value="appearance.showHeader"
      :label="t('modules.combatJournal.design.showHeader')"
      @update:model-value="emit('patch', { showHeader: $event })"
    />
    <template v-if="section === JournalSettingsSection.Rows">
      <UiField :label="t('modules.combatJournal.design.font')"
        ><UiSelect
          :model-value="appearance.font"
          :options="fonts"
          :label="t('modules.combatJournal.design.font')"
          @update:model-value="emit('patch', { font: $event })"
      /></UiField>
      <UiSwitch
        :model-value="appearance.barGradient"
        :label="t('modules.combatJournal.design.barGradient')"
        @update:model-value="emit('patch', { barGradient: $event })"
      />
    </template>
  </div>
</template>
<style scoped>
.journal-custom-colors {
  grid-column: 1/-1;
}
.journal-appearance-editor > :deep(.ui-switch) {
  grid-column: 1 / -1;
  flex-direction: row-reverse;
  justify-content: space-between;
  min-height: 40px;
  padding: 8px 12px;
  border: 1px solid var(--prism-border);
  border-radius: 5px;
  background: var(--prism-field);
}
.journal-appearance-editor {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
}
</style>
