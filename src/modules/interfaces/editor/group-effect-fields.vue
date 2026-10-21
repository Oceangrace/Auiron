<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import type { GroupEffectSettings } from '../model/rules/group-effect-settings';
import { defaultGroupEffectSettings } from '../model/rules/group-effect-settings';
import { TriggerUnit } from '../model/rules/trigger-unit';
import { EffectFilter } from '../model/rules/effect-filter';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
const props = defineProps<{ widget: WidgetNode; session: EditorSession }>();

const { t } = useI18n();

const settings = computed(() => props.widget.binding?.effects ?? defaultGroupEffectSettings);

const units = computed(() =>
  [TriggerUnit.Player, TriggerUnit.Target].map((value) => ({
    value,
    label: t('modules.interfaces.rules.options.' + value),
  })),
);

const filters = computed(() =>
  Object.values(EffectFilter).map((value) => ({
    value,
    label: t('modules.interfaces.rules.options.' + value),
  })),
);

const update = (patch: Partial<GroupEffectSettings>): void => {
  props.session.updateSelected((node) => ({
    ...node,
    binding:
      node.binding === null ? null : { ...node.binding, effects: { ...settings.value, ...patch } },
  }));
};
</script>
<template>
  <div class="field-grid">
    <UiField :label="t('modules.interfaces.rules.unit')">
      <UiSelect
        :model-value="settings.unit"
        :options="units"
        :label="t('modules.interfaces.rules.unit')"
        @update:model-value="update({ unit: $event })"
      />
    </UiField>
    <UiField :label="t('modules.interfaces.rules.filter')">
      <UiSelect
        :model-value="settings.filter"
        :options="filters"
        :label="t('modules.interfaces.rules.filter')"
        @update:model-value="update({ filter: $event })"
      />
    </UiField>
    <UiField :label="t('modules.interfaces.effectGroups.iconSize')">
      <UiNumber
        :model-value="settings.iconSize"
        :min="8"
        :max="256"
        :label="t('modules.interfaces.effectGroups.iconSize')"
        @update:model-value="update({ iconSize: $event })"
      />
    </UiField>
  </div>
</template>
