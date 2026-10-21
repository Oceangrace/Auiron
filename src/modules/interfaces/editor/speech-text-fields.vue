<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
defineProps<{ modelValue: string }>();

const emit = defineEmits<{ 'update:modelValue': [text: string] }>();

const { t } = useI18n();

const options = computed(() => [
  { value: '', label: t('modules.interfaces.audio.phrasesTitle') },
  ...['move', 'dodge', 'interrupt', 'spread', 'stack', 'dispel', 'health', 'ready', 'target'].map(
    (key) => ({
      value: t('modules.interfaces.audio.phrases.' + key),
      label: t('modules.interfaces.audio.phrases.' + key),
    }),
  ),
]);
</script>
<template>
  <UiField :label="t('modules.interfaces.audio.phrase')"
    ><UiInput
      :model-value="modelValue"
      :label="t('modules.interfaces.audio.phrase')"
      @update:model-value="emit('update:modelValue', $event)"
  /></UiField>
  <UiSelect
    model-value=""
    :options="options"
    :label="t('modules.interfaces.audio.phrasesTitle')"
    @update:model-value="
      (value) => {
        if (value !== '') {
          emit('update:modelValue', value);
        }
      }
    "
  />
</template>
