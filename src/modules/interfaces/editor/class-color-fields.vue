<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import type { WidgetStyle } from '../model/widget-style';
import { ClassColorSource } from '../model/class-color-source';
import { ClassColorTarget } from '../model/class-color-target';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';

const props = defineProps<{ widget: WidgetNode; session: EditorSession }>();

const { t } = useI18n();

const sources = computed(() =>
  Object.values(ClassColorSource).map((value) => ({
    value,
    label: t('modules.interfaces.classColor.sources.' + value),
  })),
);

const targets = computed(() =>
  Object.values(ClassColorTarget).map((value) => ({
    value,
    label: t('modules.interfaces.classColor.targets.' + value),
  })),
);

const update = (patch: Partial<WidgetStyle>): void => {
  props.session.updateSelected((node) => ({ ...node, style: { ...node.style, ...patch } }));
};
</script>
<template>
  <section class="inspector-card">
    <h3>{{ t('modules.interfaces.classColor.title') }}</h3>
    <UiField :label="t('modules.interfaces.classColor.source')">
      <UiSelect
        :model-value="widget.style.classColorSource"
        :options="sources"
        :label="t('modules.interfaces.classColor.source')"
        @update:model-value="update({ classColorSource: $event as ClassColorSource })"
      />
    </UiField>
    <UiField
      v-if="widget.style.classColorSource !== ClassColorSource.None"
      :label="t('modules.interfaces.classColor.target')"
    >
      <UiSelect
        :model-value="widget.style.classColorTarget"
        :options="targets"
        :label="t('modules.interfaces.classColor.target')"
        @update:model-value="update({ classColorTarget: $event as ClassColorTarget })"
      />
    </UiField>
  </section>
</template>
