<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { HudLocalization } from './hud-localization';
import { HudScale } from './hud-scale';
import type { EditorSession } from '../editor/editor-session';
import type { InterfaceDocument } from '../model/interface-document';
import type { WidgetPointer } from '../editor/widget-pointer';
import WidgetView from './widget-view.vue';
import type { PreviewVisual } from '../model/preview-visual';
import type { Viewport } from '../model/viewport';
import type { PreviewPlayback } from '../model/preview-playback';
import './scene.css';
const props = defineProps<{
  session: EditorSession;
  document: InterfaceDocument;
  selectedId: string | null;
  editable: boolean;
  scale: number;
  pointer: WidgetPointer | null;
  preview: boolean;
  visuals: Readonly<Record<string, PreviewVisual>>;
  viewport: Viewport | null;
  playback?: PreviewPlayback | null;
}>();

const { locale } = useI18n();

const localization = new HudLocalization();

const localized = computed(() => localization.document(props.document, locale.value));

const hud = computed(() => new HudScale(props.document.display, props.viewport));

const emit = defineEmits<{ edit: [id: string] }>();
</script>
<template>
  <div
    v-for="widget in localized.roots"
    :key="widget.id"
    class="hud-root"
    :style="{ transform: hud.transform(widget.layout) }"
  >
    <WidgetView
      :widget="widget"
      :session="session"
      :selected-id="selectedId"
      :editable="editable"
      :scale="scale * hud.factor()"
      :pointer="pointer"
      :preview="preview"
      :visuals="visuals"
      :playback="playback ?? null"
      @edit="emit('edit', $event)"
    />
  </div>
</template>
