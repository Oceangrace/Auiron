<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { CatalogClient } from '../triggers/catalog-client';
import type { CatalogItem } from '../model/catalog-item';
import type { CatalogForm } from '../model/catalog-form';
import type { CatalogSelection } from '../model/catalog-selection';
import { CatalogScope } from '../model/catalog-scope';
import CatalogPicker from '../triggers/catalog-picker.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
const props = defineProps<{
  catalog: CatalogClient;
  effect: boolean;
  selected?: readonly CatalogSelection[];
  ids?: readonly number[];
}>();

const emit = defineEmits<{
  choose: [selection: CatalogSelection, item: CatalogItem, form: CatalogForm];
}>();

const { t, locale } = useI18n();

const restored = ref<CatalogSelection | null>(null);

let revision = 0;

const selected = computed(() => props.selected?.[0] ?? restored.value);

const restore = async (ids: readonly number[], current: number): Promise<void> => {
  const scope = props.effect ? CatalogScope.Effects : CatalogScope.Skills;

  try {
    const page = await props.catalog.search({
      scope,
      contextKey: null,
      text: '',
      category: null,
      matchIds: ids,
    });

    if (page.items.length !== 1 || current !== revision) {
      return;
    }

    const item = page.items[0];

    const form = item?.forms[0];

    if (item !== undefined && form !== undefined) {
      restored.value = {
        catalogVersion: page.catalogVersion,
        scope,
        contextKey: null,
        entryKey: item.key,
        choiceKey: form.key,
      };
    }
  } catch {
    // Imported filters remain unchanged if their catalog name is unavailable.
  }
};

watch(
  () => [props.ids, props.effect, locale.value],
  () => {
    const current = ++revision;

    restored.value = null;

    const ids = props.ids ?? [];

    const selections = props.selected ?? [];

    if (selections.length === 0 && ids.length !== 0) {
      void restore(ids, current);
    }
  },
  { immediate: true },
);

const enemy = ref(false);

const context = ref<string | null>(null);

watch(
  () => props.selected,
  (selected) => {
    const selection = selected?.[0];

    if (selection !== undefined) {
      enemy.value =
        selection.scope === CatalogScope.EnemyEffects ||
        selection.scope === CatalogScope.EnemySkills;
      context.value = selection.contextKey;
    }
  },
  { immediate: true },
);

const scope = computed(() => {
  if (enemy.value === true) {
    return props.effect ? CatalogScope.EnemyEffects : CatalogScope.EnemySkills;
  }

  return props.effect ? CatalogScope.Effects : CatalogScope.Skills;
});

const choose = (item: CatalogItem, form: CatalogForm, version: string): void => {
  emit(
    'choose',
    {
      catalogVersion: version,
      scope: scope.value,
      contextKey: context.value,
      entryKey: item.key,
      choiceKey: form.key,
    },
    item,
    form,
  );
};
</script>
<template>
  <p v-if="(ids?.length ?? 0) > 0 && selected === null" class="hint">
    {{ t('modules.interfaces.legacyEffectFilter') }}
  </p>
  <UiSwitch v-model="enemy" :label="t('modules.interfaces.enemy')" />
  <CatalogPicker
    v-if="enemy"
    :client="catalog"
    :scope="CatalogScope.Enemies"
    :context-key="null"
    :selected-key="context"
    :label="t('modules.interfaces.chooseEnemy')"
    :enemies="true"
    @choose="context = $event.key"
  />
  <CatalogPicker
    v-if="!enemy || context !== null"
    :key="scope + String(context)"
    :client="catalog"
    :scope="scope"
    :context-key="enemy ? context : null"
    :selected-key="selected?.entryKey ?? null"
    :choice-key="selected?.choiceKey ?? null"
    :label="t(effect ? 'modules.interfaces.findEffect' : 'modules.interfaces.findSkill')"
    @choose="choose"
  />
</template>
