<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { AppearancePreview } from './appearance-preview';
import { PreviewScenario } from '../model/preview-scenario';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiMessage from '../../../renderer/components/ui/ui-message.vue';

const props = defineProps<{ preview: AppearancePreview }>();

const { t } = useI18n();

const enabled = props.preview.enabled;

const scenario = props.preview.scenario;

const members = props.preview.members;

const playing = props.preview.playing;

const error = props.preview.error;

const options = computed(() => [
  { value: '', label: t('modules.interfaces.samples.manual') },
  ...Object.values(PreviewScenario).map((value) => ({
    value,
    label: t('modules.interfaces.samples.' + value),
  })),
]);
</script>
<template>
  <section class="preview-controls" :aria-label="t('modules.interfaces.samples.title')">
    <UiSwitch
      :model-value="enabled"
      :label="t('modules.interfaces.samples.title')"
      @update:model-value="
        $event ? preview.sample(scenario ?? PreviewScenario.Combat) : preview.enable(false)
      "
    />
    <template v-if="enabled">
      <div class="preview-field preview-state">
        <span>{{ t('modules.interfaces.samples.state') }}</span>
        <UiSelect
          :model-value="scenario ?? ''"
          :options="options"
          :label="t('modules.interfaces.samples.state')"
          @update:model-value="
            $event === '' ? preview.update({}) : preview.sample($event as PreviewScenario)
          "
        />
      </div>
      <div v-if="scenario !== null" class="preview-field preview-members">
        <span>{{ t('modules.interfaces.samples.members') }}</span>
        <UiNumber
          :model-value="members"
          :min="0"
          :max="5"
          :label="t('modules.interfaces.samples.members')"
          @update:model-value="preview.sample(scenario ?? PreviewScenario.Combat, $event)"
        />
      </div>
      <UiSwitch
        v-if="scenario !== null"
        :model-value="playing"
        :label="t('modules.interfaces.samples.animate')"
        @update:model-value="preview.animate($event)"
      />
      <span class="preview-notice">{{ t('modules.interfaces.samples.notice') }}</span>
    </template>
    <UiMessage v-if="error !== null" class="preview-error" :text="error" />
  </section>
</template>
<style scoped>
.preview-controls {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  flex: 0 0 auto;
  gap: 10px 20px;
  padding: 10px 18px;
  border-bottom: 1px solid var(--prism-panel-border);
  container-type: inline-size;
}
.preview-field {
  display: inline-flex;
  align-items: center;
  flex: 0 0 auto;
  gap: 8px;
}
.preview-field > span {
  color: var(--prism-label);
  font-size: 12px;
  white-space: nowrap;
}
.preview-state :deep(.ui-select) {
  width: 180px;
}
.preview-members :deep(input[type='number']) {
  width: 64px;
}
.preview-controls :deep(.ui-switch) {
  min-height: 32px;
  white-space: nowrap;
}
.preview-notice {
  flex: 1 1 260px;
  min-width: 0;
  margin-inline-start: auto;
  text-align: right;
  font-size: 11px;
  line-height: 1.5;
  color: var(--prism-muted);
}
.preview-error {
  flex-basis: 100%;
}
@container (max-width: 900px) {
  .preview-state :deep(.ui-select) {
    width: 140px;
  }
  .preview-members :deep(input[type='number']) {
    width: 56px;
  }
  .preview-notice {
    flex-basis: 100%;
    margin-inline-start: 0;
    text-align: left;
  }
}
</style>
