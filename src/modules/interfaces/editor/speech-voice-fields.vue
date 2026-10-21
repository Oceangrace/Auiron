<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiSlider from '../../../renderer/components/ui/ui-slider.vue';
const props = defineProps<{ widget: WidgetNode; session: EditorSession }>();

const { t, locale } = useI18n();

const voices = computed(() => {
  const available = props.session.sound.voices.value
    .map((voice) => ({ value: voice.id, label: voice.name + ' · ' + voice.language }))
    .sort((left, right) => left.label.localeCompare(right.label, locale.value));

  const selected = props.widget.sounds.voice;

  if (selected !== '' && available.some((voice) => voice.value === selected) === false) {
    available.push({
      value: selected,
      label: t('modules.interfaces.audio.unavailableVoice'),
    });
  }

  return [{ value: '', label: t('modules.interfaces.audio.automaticVoice') }, ...available];
});

onMounted(() => {
  void props.session.sound.loadVoices();
});
</script>
<template>
  <div class="speech-voice-fields">
    <UiField :label="t('modules.interfaces.audio.voice')"
      ><UiSelect
        :model-value="widget.sounds.voice"
        :options="voices"
        :label="t('modules.interfaces.audio.voice')"
        @update:model-value="
          session.updateSelected((node) => ({ ...node, sounds: { ...node.sounds, voice: $event } }))
        "
    /></UiField>
    <UiField :label="t('modules.interfaces.audio.rate')"
      ><UiSlider
        :model-value="widget.sounds.voiceRate"
        :label="t('modules.interfaces.audio.rate')"
        :min="50"
        :max="200"
        :step="10"
        @update:model-value="
          session.updateSelected((node) => ({
            ...node,
            sounds: { ...node.sounds, voiceRate: $event },
          }))
        "
    /></UiField>
  </div>
</template>
<style scoped>
.speech-voice-fields {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 220px), 1fr));
  gap: 12px;
}
</style>
