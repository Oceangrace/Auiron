<script setup lang="ts">
import UiMessage from '../components/ui/ui-message.vue';
import ClassColorsSettings from './class-colors-settings.vue';
import { computed, inject } from 'vue';
import { useI18n } from 'vue-i18n';
import type { ShellController } from './shell-controller';
import { localizationContextKey } from '../localization/localization-context-key';
import { ScreenPreset } from '../../core/preferences/screen-preset';
import { OverlayAnchor } from '../../core/preferences/overlay-anchor';
import { ToolbarLayout } from '../../core/preferences/toolbar-layout';
import UiField from '../components/ui/ui-field.vue';
import UiSlider from '../components/ui/ui-slider.vue';
import UiButton from '../components/ui/ui-button.vue';
import UiSelect from '../components/ui/ui-select.vue';
import UiSwitch from '../components/ui/ui-switch.vue';
import type { ReleaseMonitor } from './release-monitor';
import ReleaseSettings from './release-settings.vue';
const props = defineProps<{
  shell: ShellController;
  hudEnabled: boolean;
  journalEnabled: boolean;
  releases: ReleaseMonitor;
}>();

const emit = defineEmits<{ reset: []; hud: []; journal: [] }>();

const anchors = Object.values(OverlayAnchor).filter((anchor) => anchor !== OverlayAnchor.Free);

const preferences = props.shell.getState();

const error = props.shell.getError();

const { t, locale } = useI18n();

const resources = inject(localizationContextKey);

if (resources === undefined) {
  throw new Error('Localization resources are missing');
}

const languages = computed(() => {
  const values = [...resources.languages].sort((a, b) =>
    a.label.localeCompare(b.label, locale.value),
  );

  if (values.some((option) => option.value === preferences.value.locale) === false) {
    values.push({
      value: preferences.value.locale,
      label: t('shell.languageUnavailable', { id: preferences.value.locale }),
    });
  }

  return values;
});

const presets = computed(() =>
  Object.values(ScreenPreset).map((value) => ({
    value,
    label: value === ScreenPreset.Auto ? t('shell.auto') : value,
  })),
);

const layouts = computed(() => [
  { value: ToolbarLayout.Horizontal, label: t('shell.horizontal') },
  { value: ToolbarLayout.Vertical, label: t('shell.vertical') },
]);
</script>
<template>
  <section class="application-settings">
    <p v-for="warning in resources.warnings" :key="warning" role="alert" class="editor-error">
      {{ warning }}
    </p>
    <p v-if="error !== null" role="alert" class="editor-error"><UiMessage :text="error" /></p>
    <section class="settings-section">
      <h3>{{ t('shell.sections.general') }}</h3>
      <UiField :label="t('shell.language')"
        ><UiSelect
          :model-value="preferences.locale"
          :options="languages"
          :label="t('shell.language')"
          @update:model-value="shell.update({ locale: $event })"
      /></UiField>
      <UiField :label="t('shell.preset')"
        ><UiSelect
          :model-value="preferences.screenPreset"
          :options="presets"
          :label="t('shell.preset')"
          @update:model-value="shell.update({ screenPreset: $event })"
      /></UiField>
    </section>
    <section class="settings-section">
      <h3>{{ t('shell.sections.modules') }}</h3>
      <UiSwitch
        :model-value="hudEnabled"
        :label="t('shell.showHud')"
        @update:model-value="emit('hud')"
      />
      <UiSwitch
        :model-value="journalEnabled"
        :label="t('shell.showJournal')"
        @update:model-value="emit('journal')"
      />
    </section>
    <section class="settings-section">
      <h3>{{ t('shell.sections.appearance') }}</h3>
      <UiField :label="t('shell.scale')"
        ><UiSlider
          :model-value="preferences.uiScale"
          :label="t('shell.scale')"
          :min="75"
          :max="175"
          :step="5"
          @update:model-value="shell.update({ uiScale: $event })"
      /></UiField>
      <UiField :label="t('shell.transparency')"
        ><UiSlider
          :model-value="preferences.backgroundTransparency"
          :label="t('shell.transparency')"
          :min="0"
          :max="50"
          @preview="shell.preview({ backgroundTransparency: $event })"
          @update:model-value="shell.update({ backgroundTransparency: $event })"
      /></UiField>
    </section>
    <section class="settings-section">
      <h3>{{ t('shell.sections.toolbar') }}</h3>
      <UiSwitch
        :model-value="preferences.toolbarAutoHide"
        :label="t('shell.toolbarAutoHide')"
        @update:model-value="shell.update({ toolbarAutoHide: $event })"
      />
      <UiField v-if="!preferences.toolbarAutoHide" :label="t('shell.toolbarPosition')"
        ><div class="position-grid">
          <button
            v-for="anchor in anchors"
            :key="anchor"
            type="button"
            :aria-label="t('shell.positions.' + anchor)"
            :title="t('shell.positions.' + anchor)"
            :aria-pressed="preferences.toolbarAnchor === anchor"
            @click="shell.update({ toolbarAnchor: anchor, toolbarPosition: null })"
          >
            ▪
          </button>
        </div></UiField
      >
      <UiField v-if="!preferences.toolbarAutoHide" :label="t('shell.layout')"
        ><UiSelect
          :model-value="preferences.toolbarLayout"
          :options="layouts"
          :label="t('shell.layout')"
          @update:model-value="shell.update({ toolbarLayout: $event })"
      /></UiField>
      <UiSwitch
        :model-value="preferences.toolbarShowClock"
        :label="t('shell.clock')"
        @update:model-value="shell.update({ toolbarShowClock: $event })"
      />
      <UiSwitch
        :model-value="preferences.toolbarShowPlayTime"
        :label="t('shell.playTime')"
        @update:model-value="shell.update({ toolbarShowPlayTime: $event })"
      />
      <UiButton
        @click="
          shell.update({
            toolbarAnchor: OverlayAnchor.TopCenter,
            toolbarPosition: null,
            settingsPanel: null,
          });
          emit('reset');
        "
        >{{ t('shell.reset') }}</UiButton
      >
    </section>
    <ClassColorsSettings :shell="shell" />
    <section class="settings-section">
      <h3>{{ t('shell.updates.title') }}</h3>
      <ReleaseSettings :releases="releases" />
    </section>
  </section>
</template>
<style scoped>
.application-settings {
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding: 24px;
  overflow: auto;
  width: 100%;
}
.settings-section {
  display: flex;
  flex-direction: column;
  min-width: 0;
  gap: 18px;
}
.settings-section + .settings-section {
  padding-top: 24px;
  border-top: 1px solid var(--prism-panel-border);
}
.settings-section > h3 {
  margin: 0;
  color: var(--prism-text);
  font-size: 13px;
  font-weight: 600;
}
.settings-section > .ui-button {
  align-self: flex-start;
}
.position-grid {
  display: grid;
  grid-template-columns: repeat(3, 2.2rem);
  gap: 0.3rem;
}
.position-grid button {
  height: 2.2rem;
  padding: 0;
  border: 1px solid var(--prism-border);
  background: var(--prism-field);
  color: var(--prism-muted);
  cursor: pointer;
}
.position-grid button[aria-pressed='true'] {
  color: var(--prism-text);
  border-color: var(--prism-accent);
  background: var(--prism-selected);
}
.position-grid button:hover {
  border-color: var(--prism-field-border);
  background: var(--prism-raised);
  color: var(--prism-text);
}
</style>
