<script setup lang="ts">
import { computed, onBeforeUnmount } from 'vue';
import { useI18n } from 'vue-i18n';
import FloatingSurface from '../../../renderer/shell/floating-surface.vue';
import UiIconButton from '../../../renderer/components/ui/ui-icon-button.vue';
import { LayersLayout } from './layers-layout';
import { LayersPlacement } from './layers-placement';
import { LayersResize } from './layers-resize';
import LayersPlacementMenu from './layers-placement-menu.vue';

const props = defineProps<{
  layout: LayersLayout;
  dockTarget: HTMLElement;
  selectedCount: number;
}>();

const emit = defineEmits<{ close: [] }>();

const { t } = useI18n();

const state = props.layout.getState();

const floating = computed(() => state.value.placement === LayersPlacement.Floating);

const resize = new LayersResize(props.layout);

onBeforeUnmount(() => {
  resize.end();
});
</script>
<template>
  <Teleport :to="dockTarget" :disabled="floating">
    <FloatingSurface
      v-if="floating"
      class="layers-floating-surface"
      :title="t('modules.interfaces.layers')"
      :x="state.floating.x"
      :y="state.floating.y"
      :width="state.floating.width"
      :height="state.floating.height"
      :min-width="240"
      @moved="layout.moveFloating($event)"
      @resized="layout.resizeFloating($event)"
      @close="emit('close')"
    >
      <template #toolbar>
        <small v-if="selectedCount > 1" class="layers-selection-count" role="status">
          {{ t('modules.interfaces.selectedCount', { count: selectedCount }) }}
        </small>
        <LayersPlacementMenu :placement="state.placement" @change="layout.place($event)" />
      </template>
      <slot />
    </FloatingSurface>
    <section v-else class="layers-docked-panel" :aria-label="t('modules.interfaces.layers')">
      <header class="layers-heading">
        <strong>{{ t('modules.interfaces.layers') }}</strong>
        <small v-if="selectedCount > 1" class="layers-selection-count" role="status">
          {{ t('modules.interfaces.selectedCount', { count: selectedCount }) }}
        </small>
        <LayersPlacementMenu :placement="state.placement" @change="layout.place($event)" />
        <UiIconButton :label="t('modules.interfaces.cancel')" @click="emit('close')">
          <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m5 5 10 10 M15 5 5 15" /></svg>
        </UiIconButton>
      </header>
      <slot />
      <div
        class="layers-divider"
        role="separator"
        tabindex="0"
        aria-orientation="vertical"
        :aria-label="t('modules.interfaces.layersLayout.resize')"
        :aria-valuenow="state.width"
        :aria-valuemin="LayersLayout.MIN_WIDTH"
        :aria-valuemax="LayersLayout.MAX_WIDTH"
        @pointerdown="resize.begin($event)"
        @pointermove="resize.move($event)"
        @pointerup="resize.end()"
        @pointercancel="resize.end()"
        @lostpointercapture="resize.end()"
        @keydown="resize.key($event)"
      />
    </section>
  </Teleport>
</template>
