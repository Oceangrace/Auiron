<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import { LayersPlacement } from './layers-placement';

defineProps<{ placement: LayersPlacement }>();

const emit = defineEmits<{ change: [placement: LayersPlacement] }>();

const { t } = useI18n();

const options = computed(() =>
  Object.values(LayersPlacement).map((value) => ({
    value,
    label: t('modules.interfaces.layersLayout.' + value),
    group: t('modules.interfaces.layersLayout.title'),
  })),
);
</script>
<template>
  <UiSelect
    class="layers-placement-menu"
    :model-value="placement"
    :options="options"
    :label="t('modules.interfaces.layersLayout.title')"
    :title="t('modules.interfaces.layersLayout.title')"
    display-label="⋯"
    :compact="true"
    :popup-min-width="210"
    @update:model-value="emit('change', $event)"
  />
</template>
