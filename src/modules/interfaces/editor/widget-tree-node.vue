<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import UiIconButton from '../../../renderer/components/ui/ui-icon-button.vue';
import WidgetKindIcon from './widget-kind-icon.vue';
const props = withDefaults(
  defineProps<{
    widget: WidgetNode;
    session: EditorSession;
    documentId: string;
    search?: string;
  }>(),
  { search: '' },
);

const emit = defineEmits<{
  select: [id: string, additive: boolean];
  duplicate: [id: string];
  remove: [id: string];
}>();

const state = props.session.getState();

const selected = computed(
  () =>
    state.value.documentId === props.documentId &&
    state.value.selectedIds.includes(props.widget.id),
);

const { t } = useI18n();

const expanded = computed(() => props.session.layers.isExpanded(props.documentId, props.widget.id));

const visible = computed(() =>
  props.session.tree
    .flatten([props.widget])
    .some((node) => node.name.toLowerCase().includes(props.search)),
);
</script>
<template>
  <div v-if="visible" class="tree-node">
    <div
      class="tree-node-row"
      :class="{ 'is-selected': selected, 'is-disabled': widget.enabled === false }"
    >
      <button
        v-if="session.tree.isGroup(widget)"
        type="button"
        class="tree-fold"
        :aria-label="widget.name"
        :aria-expanded="expanded || search !== ''"
        @click="session.layers.toggle(documentId, widget.id)"
      >
        <svg viewBox="0 0 20 20" aria-hidden="true">
          <path :d="expanded || search !== '' ? 'm6 8 4 4 4-4' : 'm8 6 4 4-4 4'" />
        </svg></button
      ><span v-else class="tree-fold-spacer" />
      <button
        type="button"
        class="tree-node-name"
        :aria-pressed="selected"
        @click="emit('select', widget.id, $event.shiftKey)"
      >
        <WidgetKindIcon :kind="widget.kind" /><span>{{ widget.name }}</span>
      </button>
      <div class="node-actions">
        <UiIconButton
          :label="t('modules.interfaces.duplicate')"
          :disabled="session.isReadOnly()"
          @click="emit('duplicate', widget.id)"
        >
          <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M7 7h10v10H7z M3 13V3h10" /></svg>
        </UiIconButton>
        <UiIconButton
          :label="t('modules.interfaces.remove')"
          :disabled="session.isReadOnly()"
          @click="emit('remove', widget.id)"
        >
          <svg viewBox="0 0 20 20" aria-hidden="true">
            <path d="M4 6h12 M7 6V3h6v3 M5 6l1 11h8l1-11 M8 9v5 M12 9v5" />
          </svg>
        </UiIconButton>
      </div>
    </div>
    <div v-if="expanded || search !== ''" class="tree-children">
      <WidgetTreeNode
        v-for="child in widget.children"
        :key="child.id"
        :widget="child"
        :session="session"
        :document-id="documentId"
        :search="widget.name.toLowerCase().includes(search) ? '' : search"
        @select="(id, additive) => emit('select', id, additive)"
        @duplicate="emit('duplicate', $event)"
        @remove="emit('remove', $event)"
      />
    </div>
  </div>
</template>
