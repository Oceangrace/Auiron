<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { CombatJournalModule } from '../combat-journal-module';
import JournalWindow from './journal-window.vue';
import JournalSettings from './journal-settings.vue';
import FloatingSurface from '../../../renderer/shell/floating-surface.vue';
const props = defineProps<{ journal: CombatJournalModule; metersVisible: boolean }>();

const { t } = useI18n();

const preferences = props.journal.getPreferences();

const settingsWindow = props.journal.getSettingsWindow();

const edited = computed(
  () => preferences.value?.windows.find((entry) => entry.id === settingsWindow.value) ?? null,
);

const settingsX = computed(() =>
  Math.max(
    0,
    Math.min(window.innerWidth - 680, (edited.value?.x ?? 0) + (edited.value?.width ?? 0) + 12),
  ),
);

const layout = computed(
  () =>
    preferences.value?.settingsPanel ?? {
      x: settingsX.value,
      y: Math.max(0, edited.value?.y ?? 0),
      width: 660,
      height: 640,
    },
);
</script>
<template>
  <template v-if="journal.isVisible() && preferences !== null"
    ><JournalWindow
      v-for="window in preferences.windows.filter((entry) => entry.visible)"
      :key="window.id"
      :window="window"
      :appearance="preferences.appearance"
      :journal="journal"
      :visible="metersVisible"
    />
    <FloatingSurface
      v-if="edited !== null"
      class="journal-settings-panel"
      :title="t('modules.combatJournal.settings')"
      :x="layout.x"
      :y="layout.y"
      :width="layout.width"
      :height="layout.height"
      :min-width="440"
      :min-height="260"
      @close="journal.closeSettings()"
      @resized="journal.savePanel('settingsPanel', $event)"
      @moved="journal.savePanel('settingsPanel', { ...layout, ...$event })"
    >
      <JournalSettings :journal="journal" :preferences="preferences" :window="edited" />
    </FloatingSurface>
  </template>
</template>
<style scoped>
.journal-settings-panel :deep(.surface-content) {
  flex-direction: column;
  overflow: hidden;
}
.journal-settings-panel :deep(.journal-settings-shell) {
  flex: 1;
  min-height: 0;
  overflow: auto;
}
</style>
