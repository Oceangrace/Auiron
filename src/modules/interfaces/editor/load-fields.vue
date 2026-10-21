<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, inject } from 'vue';
import { localizationContextKey } from '../../../renderer/localization/localization-context-key';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import { LoadState } from '../model/load-state';
import { LoadOptions } from './load-options';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import LoadLocationFields from './load-location-fields.vue';
const props = defineProps<{ widget: WidgetNode; session: EditorSession }>();

const { t, locale } = useI18n();

const resources = inject(localizationContextKey);

if (resources === undefined) {
  throw new Error('Localization resources are missing');
}

const catalog = new LoadOptions(t, () => locale.value, resources.zoneValues, resources.zoneNames);

const options = computed(() => ({
  playerClass: catalog.classes(props.widget.load.playerClass),
  zone: catalog.zones(props.widget.load.zone),
}));
</script>
<template>
  <section class="inspector-section">
    <div class="inspector-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.tabs.load') }}</h3>
        <p class="section-description">{{ t('modules.interfaces.redesign.hints.load') }}</p>
      </div>
      <UiField
        v-for="key in ['inCombat', 'inGroup'] as const"
        :key="key"
        :label="t('modules.interfaces.advanced.' + key)"
        ><UiSelect
          :model-value="widget.load[key]"
          :options="
            Object.values(LoadState).map((value) => ({
              value,
              label: t('modules.interfaces.advanced.options.' + value),
            }))
          "
          :label="t('modules.interfaces.advanced.' + key)"
          @update:model-value="
            session.updateSelected((node) => ({ ...node, load: { ...node.load, [key]: $event } }))
          "
      /></UiField>
      <UiField
        v-for="key in ['playerClass', 'zone'] as const"
        :key="key"
        :label="t('modules.interfaces.advanced.' + key)"
        ><UiSelect
          :model-value="widget.load[key]"
          :options="options[key]"
          :label="t('modules.interfaces.advanced.' + key)"
          @update:model-value="
            session.updateSelected((node) => ({ ...node, load: { ...node.load, [key]: $event } }))
          "
      /></UiField>
      <UiField :label="t('modules.interfaces.advanced.entityId')"
        ><UiInput
          :model-value="widget.entityId ?? ''"
          :label="t('modules.interfaces.advanced.entityId')"
          @update:model-value="
            session.updateSelected((node) => ({ ...node, entityId: $event === '' ? null : $event }))
          "
      /></UiField>
      <details
        class="inspector-disclosure"
        :open="
          widget.load.mapIds.length + widget.load.dungeonIds.length + widget.load.npcIds.length > 0
        "
      >
        <summary>{{ t('modules.interfaces.loadIds.advanced') }}</summary>
        <div class="inspector-block">
          <LoadLocationFields :widget="widget" :session="session" />
        </div>
      </details>
    </div>
  </section>
</template>
