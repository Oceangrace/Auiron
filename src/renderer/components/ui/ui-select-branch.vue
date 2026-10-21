<script setup lang="ts" generic="T extends string">
import { useI18n } from 'vue-i18n';
import type { VNode } from 'vue';
import type { UiSelectOption } from './ui-select-option';
import type { UiSelectSection } from './ui-select-section';
defineProps<{
  section: UiSelectSection<T>;
  selection: T;
  indices: ReadonlyMap<T, number>;
  activeIndex: number;
  id: string;
  query: string;
  level: number;
  label: string;
}>();
const emit = defineEmits<{
  choose: [index: number];
  highlight: [index: number];
  toggle: [key: string];
  more: [];
  dismiss: [];
}>();

const { t } = useI18n();

defineSlots<{ option?(props: { option: UiSelectOption<T> }): VNode[] }>();
</script>
<template>
  <section
    class="ui-select-branch"
    :data-level="level"
    role="group"
    :aria-label="section.label || label"
  >
    <button
      v-if="section.label !== ''"
      type="button"
      class="ui-select-group"
      :aria-expanded="section.expanded"
      :disabled="query.trim() !== ''"
      @pointerdown.prevent
      @click="emit('toggle', section.key)"
      @keydown.esc="emit('dismiss')"
    >
      <span aria-hidden="true">{{ section.expanded ? '▾' : '▸' }}</span
      ><span>{{ section.label }}</span
      ><small>{{ section.total }}</small>
    </button>
    <UiSelectBranch
      v-for="child in section.children"
      :id="id"
      :key="child.key"
      :section="child"
      :selection="selection"
      :indices="indices"
      :active-index="activeIndex"
      :query="query"
      :level="level + 1"
      :label="label"
      @choose="emit('choose', $event)"
      @highlight="emit('highlight', $event)"
      @toggle="emit('toggle', $event)"
      @more="emit('more')"
      @dismiss="emit('dismiss')"
    >
      <template #option="{ option }">
        <slot name="option" :option="option"
          ><span>{{ option.label }}</span></slot
        >
      </template>
    </UiSelectBranch>
    <button
      v-for="option in section.options"
      :id="`${id}-${indices.get(option.value) ?? -1}`"
      :key="option.value"
      type="button"
      class="ui-select-option"
      role="option"
      tabindex="-1"
      :data-value="option.value"
      :data-highlighted="activeIndex === indices.get(option.value)"
      :aria-selected="selection === option.value"
      @pointerdown.prevent
      @pointermove="emit('highlight', indices.get(option.value) ?? -1)"
      @click="emit('choose', indices.get(option.value) ?? -1)"
    >
      <slot name="option" :option="option"
        ><span>{{ option.label }}</span></slot
      >
      <svg v-if="selection === option.value" viewBox="0 0 20 20" aria-hidden="true">
        <path d="m4 10 4 4 8-8" />
      </svg>
    </button>
    <button
      v-if="
        section.expanded && section.children.length === 0 && section.options.length < section.total
      "
      type="button"
      class="ui-select-more"
      @pointerdown.prevent
      @click="emit('more')"
    >
      {{ t('select.more') }}
    </button>
  </section>
</template>
