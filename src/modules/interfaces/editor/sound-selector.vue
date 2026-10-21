<script setup lang="ts">
import { computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { SoundCue } from '../model/sound-cue';
import type { SoundPlayer } from '../scene/sound-player';
import { SoundLibrary } from '../scene/sound-library';
import type { SoundPlayback } from '../scene/sound-playback';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
const props = withDefaults(
  defineProps<{
    modelValue: SoundCue;
    label: string;
    player: SoundPlayer;
    volume: number;
    text?: string;
    voice?: string;
    rate?: number;
  }>(),
  { text: '', voice: '', rate: 100 },
);

const emit = defineEmits<{ 'update:modelValue': [cue: SoundCue] }>();

const { t } = useI18n();

const id = crypto.randomUUID();

const library = new SoundLibrary();

const options = computed(() => library.options(t));

const playing = computed(() => props.player.playing.value === id);

const request = computed<SoundPlayback>(() => ({
  cue: props.modelValue,
  volume: props.volume,
  text: props.text,
  voice: props.voice,
  rate: props.rate,
  owner: id,
}));

const preparing = computed(
  () =>
    props.modelValue === SoundCue.Speech &&
    props.text.trim() !== '' &&
    props.volume > 0 &&
    props.player.isReady(request.value) === false &&
    props.player.error.value === null,
);

const disabled = computed(
  () =>
    props.modelValue === SoundCue.None ||
    props.volume === 0 ||
    preparing.value ||
    (props.modelValue === SoundCue.Speech && props.text.trim() === ''),
);

watch(
  [request, preparing],
  () => {
    if (preparing.value === true) {
      void props.player.prepareSpeech(request.value);
    }
  },
  { immediate: true },
);

const preview = (): void => {
  if (playing.value === true) {
    props.player.stop();

    return;
  }

  void props.player.play(request.value);
};
</script>
<template>
  <UiField :label="label">
    <div class="sound-controls">
      <UiSelect
        :model-value="modelValue"
        :options="options"
        :label="label"
        @update:model-value="emit('update:modelValue', $event)"
      />
      <UiButton
        class="sound-preview"
        :disabled="disabled"
        :aria-label="
          t('modules.interfaces.audio.' + (playing ? 'stop' : 'preview')) + ' · ' + label
        "
        :title="t('modules.interfaces.audio.' + (playing ? 'stop' : 'preview'))"
        @click="preview"
      >
        <svg v-if="playing" width="12" height="12" viewBox="0 0 16 16" aria-hidden="true">
          <path fill="currentColor" d="M3 3h10v10H3z" />
        </svg>
        <svg v-else width="12" height="12" viewBox="0 0 16 16" aria-hidden="true">
          <path fill="currentColor" d="m4 2 9 6-9 6z" />
        </svg>
      </UiButton>
    </div>
    <small v-if="preparing" role="status">{{ t('modules.interfaces.audio.preparing') }}</small>
  </UiField>
</template>
<style scoped>
.sound-controls {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 28px;
  gap: 6px;
  align-items: center;
  min-width: 0;
}
.sound-controls .sound-preview {
  align-self: center;
  width: 28px;
  min-width: 28px;
  height: 32px;
  min-height: 32px;
  padding: 0;
  display: grid;
  place-items: center;
}
</style>
