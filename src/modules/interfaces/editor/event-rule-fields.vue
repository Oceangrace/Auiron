<script setup lang="ts">
import type { CatalogClient } from '../triggers/catalog-client';
import type { CatalogItem } from '../model/catalog-item';
import { CatalogScope } from '../model/catalog-scope';
import CatalogPicker from '../triggers/catalog-picker.vue';
import { useI18n } from 'vue-i18n';
import type { EventTrigger } from '../model/rules/event-trigger';
import { GameEventType } from '../model/rules/game-event-type';
import { EventParticipant } from '../model/rules/event-participant';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
const props = defineProps<{ trigger: EventTrigger; catalog: CatalogClient }>();

const emit = defineEmits<{ change: [trigger: EventTrigger] }>();

const { t } = useI18n();

const chooseNpc = (item: CatalogItem): void => {
  update({ sourceActorKey: item.key, sourceNpcId: null });
};

const update = (patch: Partial<EventTrigger>): void => {
  emit('change', { ...props.trigger, ...patch });
};

const options = <T extends string>(
  entries: Readonly<Record<string, T>>,
): { value: T; label: string }[] =>
  Object.values(entries).map((value) => ({
    value,
    label: t('modules.interfaces.rules.options.' + value),
  }));
</script>
<template>
  <UiField :label="t('modules.interfaces.rules.event')"
    ><UiSelect
      :model-value="trigger.event"
      :options="options(GameEventType)"
      :label="t('modules.interfaces.rules.event')"
      @update:model-value="update({ event: $event })"
  /></UiField>
  <CatalogPicker
    :client="catalog"
    :scope="CatalogScope.Enemies"
    :context-key="null"
    :selected-key="trigger.sourceActorKey ?? null"
    :label="t('modules.interfaces.rules.npcSourceOptional')"
    :enemies="true"
    @choose="chooseNpc"
  />
  <UiButton
    v-if="trigger.sourceActorKey !== null && trigger.sourceActorKey !== undefined"
    @click="update({ sourceActorKey: null, sourceNpcId: null })"
    >{{ t('modules.interfaces.rules.clearNpcSource') }}</UiButton
  >
  <p v-if="trigger.event === GameEventType.CastStarted" class="field-hint">
    {{ t('modules.interfaces.rules.castDurationFromEvent') }}
  </p>
  <div class="field-grid">
    <UiField
      v-for="key in ['source', 'recipient'] as const"
      :key="key"
      :label="t('modules.interfaces.rules.' + key)"
      ><UiSelect
        :model-value="trigger[key]"
        :options="options(EventParticipant)"
        :label="t('modules.interfaces.rules.' + key)"
        @update:model-value="update({ [key]: $event })"
    /></UiField>
    <UiField
      v-if="trigger.event !== GameEventType.CastStarted"
      :label="t('modules.interfaces.rules.durationMs')"
      ><UiNumber
        :model-value="trigger.durationMs"
        :label="t('modules.interfaces.rules.durationMs')"
        :min="100"
        :max="3600000"
        @update:model-value="update({ durationMs: $event })"
    /></UiField>
    <UiField
      v-if="trigger.event !== GameEventType.CastStarted"
      :label="t('modules.interfaces.rules.delayMs')"
      ><UiNumber
        :model-value="trigger.delayMs ?? 0"
        :label="t('modules.interfaces.rules.delayMs')"
        :min="0"
        :max="3600000"
        @update:model-value="update({ delayMs: $event })"
    /></UiField>
  </div>
  <UiSwitch
    :model-value="trigger.minValue !== null"
    :label="t('modules.interfaces.rules.minValue')"
    @update:model-value="update({ minValue: $event ? 0 : null })"
  />
  <UiNumber
    v-if="trigger.minValue !== null"
    :model-value="trigger.minValue"
    :label="t('modules.interfaces.rules.minValue')"
    :min="0"
    :max="1000000000000"
    @update:model-value="update({ minValue: $event })"
  />
  <UiSwitch
    :model-value="trigger.schedule !== null && trigger.schedule !== undefined"
    :label="t('modules.interfaces.rules.schedule')"
    @update:model-value="
      update({ schedule: $event ? { bossOnly: true, firstMs: 10000, intervalMs: 30000 } : null })
    "
  />
  <template v-if="trigger.schedule !== null && trigger.schedule !== undefined">
    <UiSwitch
      :model-value="trigger.schedule.bossOnly"
      :label="t('modules.interfaces.rules.bossOnly')"
      @update:model-value="update({ schedule: { ...trigger.schedule, bossOnly: $event } })"
    />
    <UiField
      v-for="key in ['firstMs', 'intervalMs'] as const"
      :key="key"
      :label="t('modules.interfaces.rules.' + key)"
      ><UiNumber
        :model-value="trigger.schedule[key]"
        :label="t('modules.interfaces.rules.' + key)"
        :min="key === 'intervalMs' ? 100 : 0"
        :max="3600000"
        @update:model-value="update({ schedule: { ...trigger.schedule, [key]: $event } })"
    /></UiField>
  </template>
</template>
