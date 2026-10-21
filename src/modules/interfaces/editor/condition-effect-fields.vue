<script setup lang="ts">
import UiMessage from '../../../renderer/components/ui/ui-message.vue';
import { computed } from 'vue';
import SoundSelector from './sound-selector.vue';
import SpeechTextFields from './speech-text-fields.vue';
import SpeechVoiceFields from './speech-voice-fields.vue';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import { useI18n } from 'vue-i18n';
import { ConditionProperty } from '../model/condition-property';
import { WidgetAnimation } from '../model/widget-animation';
import { GlowMode } from '../model/glow-mode';
import { SoundCue } from '../model/sound-cue';
import { AnimationWhen } from '../model/animation-when';
import { AnimationChange } from '../model/animation-change';
import type { ConditionChange } from '../model/condition-change';
import type { ConditionAnimation } from '../model/condition-animation';
import { defaultConditionAnimation } from '../model/condition-animation';
import { ConditionChanges } from '../model/condition-changes';
import { FontWeight } from '../model/font-weight';
import { OutlineStyle } from '../model/outline-style';
import { AnimationNumberField } from '../model/animation-number-field';
import UiColor from '../../../renderer/components/ui/ui-color.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
const props = defineProps<{
  change: ConditionChange;
  widget: WidgetNode;
  session: EditorSession;
}>();

const emit = defineEmits<{ change: [value: ConditionChange] }>();

const changeValue = (value: string | number | boolean | null): void => {
  emit('change', conditionChanges.withValue(props.change, value));
};

const changeSpeech = (text: string): void => {
  emit('change', conditionChanges.withSpeech(props.change, text));
};

const textValue = computed(() =>
  typeof props.change.value === 'string' ? props.change.value : '',
);

const speechText = computed(() =>
  props.change.property === ConditionProperty.Sound ? (props.change.speechText ?? '') : '',
);

const conditionChanges = new ConditionChanges();

const { t } = useI18n();

const cues = new Map<string, SoundCue>(Object.values(SoundCue).map((value) => [value, value]));

const cue = computed(() => cues.get(textValue.value) ?? SoundCue.None);

const speech = computed(() => cue.value === SoundCue.Speech);

const colors = [
  ConditionProperty.Color,
  ConditionProperty.FillColor,
  ConditionProperty.Background,
  ConditionProperty.BorderColor,
  ConditionProperty.TextOutlineColor,
  ConditionProperty.IconGlowColor,
];

const flags = [
  ConditionProperty.Glow,
  ConditionProperty.Desaturate,
  ConditionProperty.IconSwipe,
  ConditionProperty.BarSpark,
  ConditionProperty.BarInverse,
];

const options = computed(() => {
  let values: string[] = [];

  const property = props.change.property;

  if (property === ConditionProperty.Animation) {
    values = Object.values(WidgetAnimation);
  } else if (property === ConditionProperty.IconGlow) {
    values = Object.values(GlowMode);
  } else if (property === ConditionProperty.TextWeight) {
    values = Object.values(FontWeight);
  } else if (property === ConditionProperty.TextOutline) {
    values = Object.values(OutlineStyle);
  }

  return values.map((value) => ({
    value,
    label: t('modules.interfaces.advanced.options.' + value),
  }));
});

const config = computed(() =>
  props.change.property === ConditionProperty.AnimationConfig
    ? props.change.value
    : defaultConditionAnimation,
);

const configChange = (patch: Partial<ConditionAnimation>): void => {
  emit('change', conditionChanges.withAnimation(props.change, { ...config.value, ...patch }));
};

const labels = <Value extends string>(
  values: Readonly<Record<string, Value>>,
): { value: Value; label: string }[] =>
  Object.values(values).map((value) => ({
    value,
    label: t('modules.interfaces.advanced.options.' + value),
  }));

const presetOptions = computed(() => labels(WidgetAnimation));

const whenOptions = computed(() => labels(AnimationWhen));

const changeOptions = computed(() => labels(AnimationChange));

const numbers: Readonly<
  Record<AnimationNumberField, { readonly min: number; readonly max: number }>
> = {
  [AnimationNumberField.PeriodMs]: { min: 100, max: 10000 },
  [AnimationNumberField.Intensity]: { min: 0, max: 100 },
  [AnimationNumberField.ThresholdSeconds]: { min: 0, max: 3600 },
  [AnimationNumberField.DurationMs]: { min: 100, max: 60000 },
};
</script>
<template>
  <div v-if="change.property === ConditionProperty.Sound" class="condition-sound">
    <SoundSelector
      class="condition-sound-selector"
      :model-value="cue"
      :label="t('modules.interfaces.audio.conditionSound')"
      :player="session.sound"
      :volume="widget.sounds.volume"
      :text="speechText"
      :voice="widget.sounds.voice"
      :rate="widget.sounds.voiceRate"
      @update:model-value="changeValue($event)"
    />
    <template v-if="speech">
      <SpeechTextFields :model-value="speechText" @update:model-value="changeSpeech($event)" />
      <SpeechVoiceFields :widget="widget" :session="session" />
    </template>
    <p v-if="session.sound.error.value !== null" class="error-note" role="alert">
      <UiMessage :text="session.sound.error.value" />
    </p>
  </div>
  <div v-else-if="change.property === ConditionProperty.BarGradient" class="condition-gradient">
    <UiSwitch
      :model-value="change.value !== null"
      :label="t('modules.interfaces.enabled')"
      @update:model-value="changeValue($event === true ? widget.style.fillColor : null)"
    />
    <UiColor
      v-if="change.value !== null"
      :model-value="change.value"
      :label="t('modules.interfaces.conditions.value')"
      @update:model-value="changeValue($event)"
    />
  </div>
  <UiColor
    v-else-if="colors.includes(change.property)"
    :model-value="textValue"
    :label="t('modules.interfaces.conditions.value')"
    @update:model-value="changeValue($event)"
  />
  <UiSwitch
    v-else-if="flags.includes(change.property)"
    :model-value="change.value === true"
    :label="t('modules.interfaces.enabled')"
    @update:model-value="changeValue($event)"
  />
  <UiSelect
    v-else-if="options.length > 0"
    :model-value="textValue"
    :options="options"
    :label="t('modules.interfaces.conditions.value')"
    @update:model-value="changeValue($event)"
  />
  <div v-else-if="change.property === ConditionProperty.AnimationConfig" class="animation-fields">
    <UiField :label="t('modules.interfaces.animationConfig.preset')">
      <UiSelect
        :model-value="config.preset"
        :options="presetOptions"
        :label="t('modules.interfaces.animationConfig.preset')"
        @update:model-value="configChange({ preset: $event })"
      />
    </UiField>
    <UiField :label="t('modules.interfaces.animationConfig.when')">
      <UiSelect
        :model-value="config.when"
        :options="whenOptions"
        :label="t('modules.interfaces.animationConfig.when')"
        @update:model-value="configChange({ when: $event })"
      />
    </UiField>
    <UiField :label="t('modules.interfaces.animationConfig.change')">
      <UiSelect
        :model-value="config.change"
        :options="changeOptions"
        :label="t('modules.interfaces.animationConfig.change')"
        @update:model-value="configChange({ change: $event })"
      />
    </UiField>
    <UiField :label="t('modules.interfaces.color')">
      <UiColor
        :model-value="config.color"
        :label="t('modules.interfaces.color')"
        @update:model-value="configChange({ color: $event })"
      />
    </UiField>
    <UiField
      v-for="(range, key) in numbers"
      :key="key"
      :label="t('modules.interfaces.animationConfig.' + key)"
    >
      <UiNumber
        :model-value="config[key]"
        :label="t('modules.interfaces.animationConfig.' + key)"
        :min="range.min"
        :max="range.max"
        @update:model-value="configChange({ [key]: $event })"
      />
    </UiField>
    <UiField :label="t('modules.interfaces.advanced.path')">
      <UiInput
        :model-value="config.path"
        :label="t('modules.interfaces.advanced.path')"
        @update:model-value="configChange({ path: $event })"
      />
    </UiField>
  </div>
  <UiNumber
    v-else-if="
      change.property === ConditionProperty.Opacity ||
      change.property === ConditionProperty.FontSize
    "
    :model-value="typeof change.value === 'number' ? change.value : 0"
    :label="t('modules.interfaces.conditions.value')"
    :min="change.property === ConditionProperty.FontSize ? 6 : 0"
    :max="change.property === ConditionProperty.FontSize ? 256 : 100"
    @update:model-value="changeValue($event)"
  />
  <UiInput
    v-else
    :model-value="textValue"
    :label="t('modules.interfaces.conditions.value')"
    :multiline="
      change.property === ConditionProperty.Text || change.property === ConditionProperty.IconLabel
    "
    @update:model-value="changeValue($event)"
  />
</template>
<style scoped>
.condition-gradient {
  display: grid;
  gap: 10px;
}

.animation-fields {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
  min-width: 0;
}
.condition-sound {
  display: grid;
  gap: 10px;
  min-width: 0;
}
.condition-sound-selector :deep(.ui-label) {
  display: none;
}
@container (max-width: 400px) {
  .animation-fields {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
