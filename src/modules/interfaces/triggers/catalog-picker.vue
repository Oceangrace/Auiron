<script setup lang="ts">
import UiMessage from '../../../renderer/components/ui/ui-message.vue';
import { computed, onBeforeUnmount, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { CatalogClient } from './catalog-client';
import type { CatalogScope } from '../model/catalog-scope';
import type { CatalogItem } from '../model/catalog-item';
import type { CatalogForm } from '../model/catalog-form';
import { CatalogPickerController } from './catalog-picker-controller';
import { CatalogChoice } from './catalog-choice';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import './catalog-picker.css';
const props = defineProps<{
  client: CatalogClient;
  scope: CatalogScope;
  contextKey: string | null;
  label: string;
  selectedKey?: string | null;
  choiceKey?: string | null;
  enemies?: boolean;
}>();

const emit = defineEmits<{ choose: [item: CatalogItem, form: CatalogForm, version: string] }>();

const { t, locale } = useI18n();

const controller = new CatalogPickerController(props.client);

const choices = new CatalogChoice();

const state = controller.getState();

const query = ref('');

const options = computed(() =>
  state.value.page.items.map((item) => ({
    value: item.key,
    label: item.context === '' ? item.name : item.name + ' · ' + item.context,
  })),
);

const label = computed(() => state.value.selected?.name ?? props.label);

watch(
  () => [props.scope, props.contextKey, query.value, locale.value],
  () => {
    controller.schedule({
      scope: props.scope,
      contextKey: props.contextKey,
      text: query.value,
      category: null,
    });
  },
  { immediate: true },
);

watch(
  () => [props.scope, props.contextKey, props.selectedKey, locale.value],
  () => {
    void controller.restore(props.scope, props.contextKey, props.selectedKey ?? null);
  },
  { immediate: true },
);

onBeforeUnmount(() => {
  controller.dispose();
});

const pick = (key: string): void => {
  const item = state.value.page.items.find((item) => item.key === key);

  if (item === undefined) {
    return;
  }

  const form = choices.all(item);

  if (form !== null) {
    controller.select(item);
    emit('choose', item, form, state.value.page.catalogVersion);
  }
};
</script>
<template>
  <div class="catalog-picker">
    <UiField :label="props.label">
      <UiSelect
        :model-value="selectedKey ?? ''"
        :display-label="label"
        :options="options"
        :label="props.label"
        :remote-search="true"
        @search="query = $event"
        @update:model-value="pick"
      />
    </UiField>
    <p v-if="state.restoring" class="hint">{{ t('modules.interfaces.loading') }}</p>
    <p v-if="state.error !== null" role="alert" class="error-note">
      <UiMessage :text="state.error" />
    </p>
    <p
      v-else-if="selectedKey != null && state.selected === null && !state.restoring"
      role="alert"
      class="error-note"
    >
      {{ t('modules.interfaces.unsupported') }}
    </p>
  </div>
</template>
