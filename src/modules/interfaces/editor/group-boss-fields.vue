<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { CatalogClient } from '../triggers/catalog-client';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';

const props = defineProps<{
  widget: WidgetNode;
  session: EditorSession;
  catalog: CatalogClient;
}>();

const { t, locale } = useI18n();

const search = ref('');

const map = ref('all');

const maps = computed(() => [
  { value: 'all', label: t('modules.interfaces.bosses.allMaps') },
  ...[...new Set(props.catalog.fieldBosses().map((boss) => boss.mapId))].map((id) => ({
    value: String(id),
    label: props.catalog.zoneName(id),
  })),
]);

const selected = computed(() => props.widget.binding?.bossIds ?? []);

const bosses = computed(() => {
  const query = search.value.trim().toLocaleLowerCase(locale.value);

  return props.catalog
    .fieldBosses()
    .filter((boss) => map.value === 'all' || String(boss.mapId) === map.value)
    .map((boss) => ({
      ...boss,
      name: props.catalog.npcName(boss.npcId),
      zone: props.catalog.zoneName(boss.mapId),
    }))
    .filter((boss) => (boss.name + ' ' + boss.zone).toLocaleLowerCase(locale.value).includes(query))
    .sort((a, b) => {
      const zone = a.zone.localeCompare(b.zone, locale.value);

      return zone === 0 ? a.name.localeCompare(b.name, locale.value) : zone;
    });
});

const update = (bossIds: readonly number[]): void => {
  props.session.updateSelected((node) => ({
    ...node,
    binding: node.binding === null ? null : { ...node.binding, bossIds },
  }));
};

const toggle = (id: number): void => {
  update(
    selected.value.includes(id)
      ? selected.value.filter((value) => value !== id)
      : [...selected.value, id],
  );
};
</script>
<template>
  <section class="boss-picker">
    <header>
      <strong>{{ t('modules.interfaces.bosses.selected', { count: selected.length }) }}</strong>
      <UiButton :disabled="selected.length === 0 || session.isReadOnly()" @click="update([])">
        {{ t('modules.interfaces.bosses.clear') }}
      </UiButton>
    </header>
    <UiSelect v-model="map" :options="maps" :label="t('modules.interfaces.bosses.map')" />
    <UiInput
      v-model="search"
      :label="t('modules.interfaces.bosses.search')"
      :placeholder="t('modules.interfaces.bosses.search')"
    />
    <div class="boss-options">
      <label v-for="boss in bosses" :key="boss.id" :class="{ chosen: selected.includes(boss.id) }">
        <input
          type="checkbox"
          :checked="selected.includes(boss.id)"
          :disabled="session.isReadOnly()"
          @change="toggle(boss.id)"
        />
        <span>{{ boss.name }}</span
        ><small>{{ boss.zone }}</small>
      </label>
      <p v-if="bosses.length === 0">{{ t('modules.interfaces.bosses.noMatches') }}</p>
    </div>
    <p>{{ t('modules.interfaces.bosses.hint') }}</p>
    <p v-if="selected.length === 0">{{ t('modules.interfaces.bosses.empty') }}</p>
  </section>
</template>
<style scoped>
.boss-picker {
  display: grid;
  gap: 10px;
}
.boss-picker header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}
.boss-picker strong {
  font-size: 13px;
}
.boss-picker p {
  margin: 0;
  color: var(--prism-muted);
  font-size: 12px;
  line-height: 1.5;
}
.boss-options {
  display: grid;
  gap: 3px;
  max-height: 280px;
  overflow-y: auto;
}
.boss-options label {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
}
.boss-options label:hover,
.boss-options label.chosen {
  background: var(--prism-surface);
}
.boss-options input {
  accent-color: var(--prism-accent);
  width: 15px;
  height: 15px;
  flex: none;
}
.boss-options span {
  flex: 1;
  font-size: 13px;
}
.boss-options small {
  flex: none;
  padding: 3px 6px;
  border: 1px solid var(--prism-border);
  border-radius: 3px;
  color: var(--prism-accent);
  font-size: 11px;
}
</style>
