<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import { LoadIdKind } from '../model/load-id-kind';
import { GameIdList } from './game-id-list';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';

const props = defineProps<{ widget: WidgetNode; session: EditorSession }>();

const { t } = useI18n();

const parser = new GameIdList();

const invalid = ref<LoadIdKind | null>(null);

const update = (key: LoadIdKind, value: string): void => {
  const ids = parser.parse(value, true, 512);

  invalid.value = ids === null ? key : null;

  if (ids !== null) {
    props.session.updateSelected((node) => ({ ...node, load: { ...node.load, [key]: ids } }));
  }
};
</script>
<template>
  <UiField
    v-for="key in Object.values(LoadIdKind)"
    :key="key"
    :label="t('modules.interfaces.loadIds.' + key)"
  >
    <UiInput
      :model-value="(widget.load[key] ?? []).join(', ')"
      :label="t('modules.interfaces.loadIds.' + key)"
      multiline
      @update:model-value="update(key, $event)"
    />
    <p v-if="invalid === key" role="alert">{{ t('modules.interfaces.loadIds.invalid') }}</p>
  </UiField>
</template>
