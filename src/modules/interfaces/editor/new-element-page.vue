<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { EditorSession } from './editor-session';
import { WidgetKind } from '../model/widget-kind';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import WidgetKindIcon from './widget-kind-icon.vue';
const props = defineProps<{ session: EditorSession; parentId: string | null }>();

const emit = defineEmits<{ done: [] }>();

const { t } = useI18n();

const state = props.session.getState();

const destination = ref(props.parentId ?? '');

const sections = {
  display: [WidgetKind.Text, WidgetKind.Icon, WidgetKind.Bar],
  layout: [WidgetKind.Group, WidgetKind.DynamicGroup],
};

const groups = computed(() =>
  props.session.tree
    .flatten(props.session.document()?.roots ?? [])
    .filter((node) => props.session.tree.isGroup(node))
    .map((node) => ({
      value: node.id,
      label: props.session.tree
        .path(props.session.document()?.roots ?? [], node.id)
        .map((entry) => entry.name)
        .join(' › '),
    })),
);

const destinations = computed(() => [
  { value: '', label: props.session.document()?.name ?? '' },
  ...groups.value,
]);

const add = (kind: WidgetKind): void => {
  if (props.session.document() === null) {
    props.session.createDocument(t('modules.interfaces.defaultInterface'));
  }

  props.session.createWidget(
    kind,
    t('modules.interfaces.types.' + kind),
    destination.value === '' ? null : destination.value,
  );
  emit('done');
};

const createInterface = (): void => {
  props.session.createDocument(t('modules.interfaces.defaultInterface'));
  emit('done');
};
</script>
<template>
  <section class="new-element-page">
    <header class="create-heading">
      <h1>{{ t('modules.interfaces.create.elementTitle') }}</h1>
      <p>{{ t('modules.interfaces.create.hint') }}</p>
    </header>
    <UiField
      v-if="session.document() !== null"
      class="element-destination"
      :label="t('modules.interfaces.create.destination')"
    >
      <UiSelect
        v-model="destination"
        :options="destinations"
        :label="t('modules.interfaces.create.destination')"
      />
    </UiField>
    <div class="element-palette">
      <section v-for="(kinds, section) in sections" :key="section">
        <h2>{{ t('modules.interfaces.create.sections.' + section) }}</h2>
        <div class="element-tiles">
          <button
            v-for="kind in kinds"
            :key="kind"
            type="button"
            class="element-tile"
            :disabled="state.readOnly"
            :data-kind="kind"
            @click="add(kind)"
          >
            <WidgetKindIcon :kind="kind" />
            <span class="element-copy">
              <strong>{{ t('modules.interfaces.types.' + kind) }}</strong>
              <small>{{ t('modules.interfaces.create.descriptions.' + kind) }}</small>
            </span>
            <svg class="element-add" viewBox="0 0 20 20" aria-hidden="true">
              <path d="M10 3v14 M3 10h14" />
            </svg>
          </button>
        </div>
      </section>
    </div>
    <button
      type="button"
      class="create-interface"
      :disabled="state.readOnly"
      @click="createInterface"
    >
      <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M3 3h18v18H3z M12 7v10 M7 12h10" /></svg>
      <span>{{ t('modules.interfaces.create.emptyAction') }}</span>
      <svg class="create-interface-arrow" viewBox="0 0 20 20" aria-hidden="true">
        <path d="m8 5 5 5-5 5" />
      </svg>
    </button>
  </section>
</template>
<style src="./element-palette.css"></style>
