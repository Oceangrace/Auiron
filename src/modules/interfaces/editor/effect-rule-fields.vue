<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { EffectTrigger } from '../model/rules/effect-trigger';
import { TriggerUnit } from '../model/rules/trigger-unit';
import { EffectFilter } from '../model/rules/effect-filter';
import { EffectPresence } from '../model/rules/effect-presence';
import { EffectOrigin } from '../model/rules/effect-origin';
import { CloneDirection } from '../model/rules/clone-direction';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
const props = defineProps<{ trigger: EffectTrigger }>();

const emit = defineEmits<{ change: [trigger: EffectTrigger] }>();

const { t } = useI18n();

const options = <T extends string>(
  entries: Readonly<Record<string, T>>,
): { value: T; label: string }[] =>
  Object.values(entries).map((value) => ({
    value,
    label: t('modules.interfaces.rules.options.' + value),
  }));

const update = (patch: Partial<EffectTrigger>): void => {
  emit('change', { ...props.trigger, ...patch });
};
</script>
<template>
  <div class="field-grid">
    <UiField :label="t('modules.interfaces.rules.unit')"
      ><UiSelect
        :model-value="trigger.unit"
        :options="options(TriggerUnit)"
        :label="t('modules.interfaces.rules.unit')"
        @update:model-value="update({ unit: $event })"
    /></UiField>
    <UiField
      v-if="trigger.unit === TriggerUnit.Member"
      :label="t('modules.interfaces.rules.member')"
      ><UiInput
        :model-value="trigger.member"
        :label="t('modules.interfaces.rules.member')"
        @update:model-value="update({ member: $event })"
    /></UiField>
    <UiField :label="t('modules.interfaces.rules.filter')"
      ><UiSelect
        :model-value="trigger.filter"
        :options="options(EffectFilter)"
        :label="t('modules.interfaces.rules.filter')"
        @update:model-value="update({ filter: $event })"
    /></UiField>
    <UiField :label="t('modules.interfaces.rules.presence')"
      ><UiSelect
        :model-value="trigger.presence"
        :options="options(EffectPresence)"
        :label="t('modules.interfaces.rules.presence')"
        @update:model-value="update({ presence: $event })"
    /></UiField>
    <UiField :label="t('modules.interfaces.rules.origin')"
      ><UiSelect
        :model-value="trigger.origin"
        :options="options(EffectOrigin)"
        :label="t('modules.interfaces.rules.origin')"
        @update:model-value="update({ origin: $event })"
    /></UiField>
    <UiField :label="t('modules.interfaces.rules.minStacks')"
      ><UiNumber
        :model-value="trigger.minStacks"
        :label="t('modules.interfaces.rules.minStacks')"
        :min="1"
        :max="999"
        @update:model-value="update({ minStacks: $event })"
    /></UiField>
    <UiField :label="t('modules.interfaces.rules.remainingBelowSeconds')"
      ><UiNumber
        :model-value="trigger.remainingBelowSeconds"
        :label="t('modules.interfaces.rules.remainingBelowSeconds')"
        :min="0"
        :max="3600"
        @update:model-value="update({ remainingBelowSeconds: $event })"
    /></UiField>
  </div>
  <UiSwitch
    :model-value="trigger.clones"
    :label="t('modules.interfaces.rules.clones')"
    @update:model-value="update({ clones: $event })"
  />
  <UiSelect
    v-if="trigger.clones"
    :model-value="trigger.cloneDirection"
    :options="options(CloneDirection)"
    :label="t('modules.interfaces.rules.cloneDirection')"
    @update:model-value="update({ cloneDirection: $event })"
  />
</template>
