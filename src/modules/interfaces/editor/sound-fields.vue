<script setup lang="ts">
import UiMessage from '../../../renderer/components/ui/ui-message.vue';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import { SoundCue } from '../model/sound-cue';
import SoundSelector from './sound-selector.vue';
import SpeechTextFields from './speech-text-fields.vue';
import SpeechVoiceFields from './speech-voice-fields.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSlider from '../../../renderer/components/ui/ui-slider.vue';
const props = defineProps<{ widget: WidgetNode; session: EditorSession }>();

const { t } = useI18n();

const speech = computed(
  () =>
    props.widget.sounds.show === SoundCue.Speech || props.widget.sounds.hide === SoundCue.Speech,
);
</script>
<template>
  <section class="sound-fields inspector-card">
    <div class="section-intro">
      <h3>{{ t('modules.interfaces.advanced.sounds') }}</h3>
      <p class="section-description">{{ t('modules.interfaces.redesign.hints.sound') }}</p>
    </div>
    <div class="sound-events">
      <div v-for="event in ['show', 'hide'] as const" :key="event" class="sound-event">
        <SoundSelector
          :model-value="widget.sounds[event]"
          :label="t('modules.interfaces.advanced.sound.' + event)"
          :player="session.sound"
          :volume="widget.sounds.volume"
          :text="event === 'show' ? widget.sounds.showText : widget.sounds.hideText"
          :voice="widget.sounds.voice"
          :rate="widget.sounds.voiceRate"
          @update:model-value="
            session.updateSelected((node) => ({
              ...node,
              sounds: { ...node.sounds, [event]: $event },
            }))
          "
        />
        <SpeechTextFields
          v-if="widget.sounds[event] === SoundCue.Speech"
          :model-value="event === 'show' ? widget.sounds.showText : widget.sounds.hideText"
          @update:model-value="
            session.updateSelected((node) => ({
              ...node,
              sounds: { ...node.sounds, [event === 'show' ? 'showText' : 'hideText']: $event },
            }))
          "
        />
      </div>
    </div>
    <SpeechVoiceFields v-if="speech" :widget="widget" :session="session" />
    <UiField :label="t('modules.interfaces.advanced.volume')"
      ><UiSlider
        :model-value="widget.sounds.volume"
        :label="t('modules.interfaces.advanced.volume')"
        @update:model-value="
          session.updateSelected((node) => ({
            ...node,
            sounds: { ...node.sounds, volume: $event },
          }))
        "
    /></UiField>
    <p v-if="session.sound.error.value !== null" class="error-note" role="alert">
      <UiMessage :text="session.sound.error.value" />
    </p>
  </section>
</template>
<style scoped>
.sound-fields,
.sound-event {
  display: grid;
  gap: 10px;
  min-width: 0;
}
.sound-events {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(100%, 240px), 1fr));
  gap: 12px;
  align-items: start;
}
.sound-fields h3 {
  margin: 6px 0 0;
}
</style>
