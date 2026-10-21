<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { EditorSession } from './editor-session';
import type { WidgetNode } from '../model/widget-node';
import type { WidgetAnchors } from '../model/widget-anchors';
import { WidgetKind } from '../model/widget-kind';
import { AnchorMode } from '../model/anchor-mode';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
const props = defineProps<{ session: EditorSession; widget: WidgetNode }>();

const { t } = useI18n();

const parent = computed(() =>
  props.session.tree.parent(props.session.document()?.roots ?? [], props.widget.id),
);

const change = (patch: Partial<WidgetAnchors>): void => {
  props.session.updateSelected((node) => ({ ...node, anchors: { ...node.anchors, ...patch } }));
};

const modes = computed(() => ({
  horizontal: Object.values(AnchorMode).map((value) => ({
    value,
    label: t('modules.interfaces.anchors.horizontalModes.' + value),
  })),
  vertical: Object.values(AnchorMode).map((value) => ({
    value,
    label: t('modules.interfaces.anchors.verticalModes.' + value),
  })),
}));
</script>
<template>
  <section v-if="parent?.kind === WidgetKind.Group" class="anchor-fields inspector-card">
    <h3>{{ t('modules.interfaces.anchors.title') }}</h3>
    <div class="field-grid">
      <UiField
        v-for="axis in ['horizontal', 'vertical'] as const"
        :key="axis"
        :label="t('modules.interfaces.anchors.' + axis)"
      >
        <UiSelect
          :model-value="widget.anchors[axis]"
          :options="modes[axis]"
          :label="t('modules.interfaces.anchors.' + axis)"
          @update:model-value="change({ [axis]: $event })"
        />
      </UiField>
      <UiField
        v-if="widget.anchors.horizontal === AnchorMode.Stretch"
        :label="t('modules.interfaces.anchors.right')"
      >
        <UiNumber
          :model-value="widget.anchors.right"
          :label="t('modules.interfaces.anchors.right')"
          :min="-100000"
          :max="100000"
          @update:model-value="change({ right: $event })"
        />
      </UiField>
      <UiField
        v-if="widget.anchors.vertical === AnchorMode.Stretch"
        :label="t('modules.interfaces.anchors.bottom')"
      >
        <UiNumber
          :model-value="widget.anchors.bottom"
          :label="t('modules.interfaces.anchors.bottom')"
          :min="-100000"
          :max="100000"
          @update:model-value="change({ bottom: $event })"
        />
      </UiField>
    </div>
    <template v-if="widget.anchors.horizontal !== AnchorMode.Stretch">
      <UiSwitch
        :model-value="widget.anchors.widthPercent !== null"
        :label="t('modules.interfaces.anchors.relativeWidth')"
        @update:model-value="change({ widthPercent: $event ? 100 : null })"
      />
      <UiField
        v-if="widget.anchors.widthPercent !== null"
        :label="t('modules.interfaces.anchors.percent')"
      >
        <UiNumber
          :model-value="widget.anchors.widthPercent"
          :label="t('modules.interfaces.anchors.percent')"
          :min="1"
          :max="200"
          @update:model-value="change({ widthPercent: $event })"
        />
      </UiField>
    </template>
    <template v-if="widget.anchors.vertical !== AnchorMode.Stretch">
      <UiSwitch
        :model-value="widget.anchors.heightPercent !== null"
        :label="t('modules.interfaces.anchors.relativeHeight')"
        @update:model-value="change({ heightPercent: $event ? 100 : null })"
      />
      <UiField
        v-if="widget.anchors.heightPercent !== null"
        :label="t('modules.interfaces.anchors.percent')"
      >
        <UiNumber
          :model-value="widget.anchors.heightPercent"
          :label="t('modules.interfaces.anchors.percent')"
          :min="1"
          :max="200"
          @update:model-value="change({ heightPercent: $event })"
        />
      </UiField>
    </template>
  </section>
</template>
<style scoped>
.anchor-fields {
  display: grid;
  gap: 14px;
}
</style>
