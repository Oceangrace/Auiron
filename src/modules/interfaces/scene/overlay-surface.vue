<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount, watch } from 'vue';
import type { InterfacesModule } from '../interfaces-module';
import SelectionGuides from './selection-guides.vue';
import AlignmentGuides from './alignment-guides.vue';
import InterfaceScene from './interface-scene.vue';
import { HudScale } from './hud-scale';
import { WidgetPointer } from '../editor/widget-pointer';
const props = defineProps<{ interfaces: InterfacesModule }>();

const state = props.interfaces.session.getState();

const snapshot = props.interfaces.getSnapshot();

const editing = props.interfaces.getEditing();

const visuals = props.interfaces.getVisuals();

const playback = props.interfaces.preview.getPlayback();

const visible = computed(() => props.interfaces.surfaceVisible());

const pointer = new WidgetPointer(props.interfaces.session);

const snapGuides = pointer.getGuides();

const cancel = (): void => {
  pointer.cancel();
};

watch(visible, (value) => {
  if (value === false) {
    cancel();
  }
});

onMounted(() => {
  window.addEventListener('blur', cancel);
});
onBeforeUnmount(() => {
  window.removeEventListener('blur', cancel);
  pointer.cancel();
});
const documents = computed(() =>
  state.value.workspace.documents.filter((document) => editing.value || document.enabled),
);

const selected = computed(() => props.interfaces.session.selected());

const selectionLayout = computed(
  () => (
    visuals.value,
    selected.value === null
      ? null
      : props.interfaces.session.tree.world(
          props.interfaces.session.document()?.roots ?? [],
          selected.value.id,
        )
  ),
);

const selectionScreen = computed(() => {
  const document = props.interfaces.session.document();

  const root =
    selected.value === null || document === null
      ? undefined
      : props.interfaces.session.tree.path(document.roots, selected.value.id)[0];

  if (selectionLayout.value === null || root === undefined || document === null) {
    return selectionLayout.value;
  }

  return new HudScale(document.display, snapshot.value?.viewport ?? null).screen(
    root.layout,
    selectionLayout.value,
  );
});
</script>
<template>
  <div v-if="visible" class="overlay-surface">
    <InterfaceScene
      v-for="document in documents"
      :key="document.id"
      :document="document"
      :session="interfaces.session"
      :selected-id="editing ? state.selectedId : null"
      :editable="editing && state.readOnly === false"
      :scale="1"
      :viewport="snapshot?.viewport ?? null"
      :pointer="pointer"
      :preview="editing"
      :visuals="visuals"
      :playback="playback"
      @edit="
        interfaces.open();
        interfaces.session.selectWidget($event);
      "
    />
    <SelectionGuides
      v-if="editing && selectionLayout !== null && selectionScreen !== null && snapshot !== null"
      :layout="selectionScreen"
      :measure="selectionLayout"
      :viewport="snapshot.viewport"
    />
    <AlignmentGuides v-if="editing" :guides="snapGuides" />
  </div>
</template>
