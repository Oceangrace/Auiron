<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { CatalogClient } from '../triggers/catalog-client';
import type { BossTrigger } from '../model/rules/boss-trigger';
import { BossEvent } from '../model/rules/boss-event';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
const props = defineProps<{ trigger: BossTrigger; catalog: CatalogClient }>();

const emit = defineEmits<{ change: [trigger: BossTrigger] }>();

const { t, locale } = useI18n();

const bosses = computed(() =>
  props.catalog
    .fieldBosses()
    .map((boss) => ({
      value: String(boss.id),
      label: props.catalog.npcName(boss.npcId) + ' · ' + props.catalog.zoneName(boss.mapId),
    }))
    .sort((a, b) => a.label.localeCompare(b.label, locale.value)),
);

const events = computed(() =>
  Object.values(BossEvent).map((value) => ({
    value,
    label: t('modules.interfaces.bossRules.events.' + value),
  })),
);

const update = (patch: Partial<BossTrigger>): void => {
  emit('change', { ...props.trigger, ...patch });
};
</script>
<template>
  <UiField :label="t('modules.interfaces.bossRules.boss')">
    <UiSelect
      :model-value="String(trigger.bossId)"
      :options="bosses"
      :label="t('modules.interfaces.bossRules.boss')"
      @update:model-value="update({ bossId: Number($event) })"
    />
  </UiField>
  <UiField :label="t('modules.interfaces.rules.event')">
    <UiSelect
      :model-value="trigger.event"
      :options="events"
      :label="t('modules.interfaces.rules.event')"
      @update:model-value="update({ event: $event })"
    />
  </UiField>
  <div class="field-grid">
    <UiField
      v-if="trigger.event === BossEvent.BeforeSpawn"
      :label="t('modules.interfaces.bossRules.beforeSeconds')"
    >
      <UiNumber
        :model-value="trigger.beforeSeconds"
        :min="0"
        :max="86400"
        :label="t('modules.interfaces.bossRules.beforeSeconds')"
        @update:model-value="update({ beforeSeconds: $event })"
      />
    </UiField>
    <UiField
      v-if="trigger.event === BossEvent.BeforeSpawn || trigger.event === BossEvent.Spawned"
      :label="t('modules.interfaces.bossRules.durationSeconds')"
    >
      <UiNumber
        :model-value="trigger.durationMs / 1000"
        :min="0.1"
        :max="3600"
        :label="t('modules.interfaces.bossRules.durationSeconds')"
        @update:model-value="update({ durationMs: $event * 1000 })"
      />
    </UiField>
  </div>
</template>
