<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { OverlayPosition } from '../../core/preferences/overlay-position';
import UiIconButton from '../components/ui/ui-icon-button.vue';
import { PanelResizeController } from './panel-resize-controller';
import { ResizeDirection } from './resize-direction';
const props = withDefaults(
  defineProps<{
    title?: string;
    x: number;
    y: number;
    width?: number;
    height?: number;
    minWidth?: number;
    minHeight?: number;
    compact?: boolean;
    attachmentWidth?: number;
    draggable?: boolean;
    interactive?: boolean;
  }>(),
  {
    title: '',
    width: 0,
    height: 0,
    minWidth: 650,
    minHeight: 260,
    compact: false,
    attachmentWidth: 0,
    draggable: true,
    interactive: true,
  },
);

const emit = defineEmits<{
  close: [];
  moved: [position: OverlayPosition];
  resized: [size: { x: number; y: number; width: number; height: number }];
}>();

const { t } = useI18n();

const collapsed = ref(false);

const position = ref({ x: props.x, y: props.y });

const size = ref({ width: props.width, height: props.height });

const root = ref<HTMLElement | null>(null);

let resize: PanelResizeController | null = null;

onMounted(() => {
  if (root.value !== null && props.compact === false) {
    resize = new PanelResizeController(root.value, (nextPosition, nextSize) => {
      position.value = nextPosition;
      size.value = nextSize;
      emit('resized', { ...nextPosition, ...nextSize });
    });
    resize.start();
  }
});

onBeforeUnmount(() => {
  resize?.dispose();
});

watch(collapsed, (value) => {
  resize?.setEnabled(value === false);
});

const gesture = ref<{
  x: number;
  y: number;
  originX: number;
  originY: number;
  width: number;
  height: number;
} | null>(null);

watch(
  () => [props.x, props.y],
  () => {
    position.value = { x: props.x, y: props.y };
  },
);
watch(
  () => [props.width, props.height],
  () => {
    size.value = { width: props.width, height: props.height };
  },
);
const attachmentActive = computed(() => props.attachmentWidth > 0 && collapsed.value === false);

const attachmentSpace = computed(() =>
  attachmentActive.value === false
    ? '0px'
    : 'calc(min(' + String(props.attachmentWidth) + 'px, 32vw) + 12px)',
);

const availableWidth = computed(() => 'calc(100vw - 24px - ' + attachmentSpace.value + ')');

const surfaceWidth = computed(
  () => 'min(' + String(size.value.width) + 'px, ' + availableWidth.value + ')',
);

const positionWidth = computed(() => (size.value.width === 0 ? '40px' : surfaceWidth.value));

const positionHeight = computed(() =>
  size.value.height === 0 || collapsed.value
    ? '40px'
    : 'min(' + String(size.value.height) + 'px, calc(100vh - 60px))',
);

const surfaceLeft = computed(() =>
  attachmentActive.value === false
    ? `clamp(0px, ${String(position.value.x)}px, calc(100vw - ${positionWidth.value}))`
    : `clamp(12px, ${String(position.value.x)}px, calc(100vw - ${attachmentSpace.value} - ${surfaceWidth.value} - 12px))`,
);

const style = computed(() => ({
  '--surface-attachment-space': attachmentSpace.value,
  left: surfaceLeft.value,
  top: `clamp(0px, ${String(position.value.y)}px, calc(100vh - ${positionHeight.value}))`,
  minWidth: props.compact
    ? undefined
    : 'min(' + String(props.minWidth) + 'px, ' + availableWidth.value + ')',
  minHeight:
    props.compact || collapsed.value
      ? undefined
      : 'min(' + String(props.minHeight) + 'px, calc(100vh - 32px))',
  width: size.value.width === 0 ? 'max-content' : surfaceWidth.value,
  height:
    collapsed.value || size.value.height === 0
      ? 'auto'
      : 'min(' + String(size.value.height) + 'px, calc(100vh - 60px))',
}));

const begin = (event: PointerEvent): void => {
  if (props.draggable === false) {
    return;
  }

  if (event.button !== 0 || event.currentTarget instanceof HTMLElement === false) {
    return;
  }

  if (
    event.target instanceof Element === true &&
    event.target.closest('button, input, select') !== null
  ) {
    return;
  }

  event.preventDefault();
  event.currentTarget.setPointerCapture(event.pointerId);
  const element = root.value;

  if (element === null) {
    return;
  }

  const rect = element.getBoundingClientRect();

  gesture.value = {
    x: event.clientX,
    y: event.clientY,
    originX: rect.x,
    originY: rect.y,
    width: rect.width,
    height: rect.height,
  };
};

const move = (event: PointerEvent): void => {
  const active = gesture.value;

  if (active === null) {
    return;
  }

  const x = event.clientX - active.x;

  const y = event.clientY - active.y;

  position.value = {
    x: Math.max(0, Math.min(window.innerWidth - active.width, active.originX + x)),
    y: Math.max(0, Math.min(window.innerHeight - 40, active.originY + y)),
  };
};

const end = (): void => {
  if (gesture.value === null) {
    return;
  }

  gesture.value = null;
  emit('moved', position.value);
};
</script>
<template>
  <section
    ref="root"
    class="floating-surface"
    :class="{ 'surface-compact': compact }"
    :style="style"
    :data-overlay-interactive="interactive ? '' : undefined"
  >
    <header
      class="surface-header"
      :inert="!interactive"
      @pointerdown="begin"
      @pointermove="move"
      @pointerup="end"
      @pointercancel="end"
      @lostpointercapture="end"
    >
      <slot name="header"
        ><slot name="leading" />
        <slot name="title"
          ><strong>{{ title }}</strong></slot
        >
        <slot name="toolbar" />
        <div class="surface-actions">
          <UiIconButton
            :label="t(collapsed ? 'shell.expandPanel' : 'shell.collapsePanel')"
            :aria-expanded="!collapsed"
            @click="collapsed = !collapsed"
          >
            <svg viewBox="0 0 20 20" aria-hidden="true">
              <path d="M5 10h10" />
              <path v-if="collapsed" d="M10 5v10" />
            </svg>
          </UiIconButton>
          <UiIconButton :label="t('shell.closePanel', { title })" @click="emit('close')">
            <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m5 5 10 10 M15 5 5 15" /></svg>
          </UiIconButton></div
      ></slot>
    </header>
    <slot name="edge" />
    <div v-if="compact === false" v-show="collapsed === false" class="surface-content">
      <slot />
    </div>
    <div v-if="attachmentActive" class="surface-attachment" data-overlay-interactive>
      <slot name="attachment" />
    </div>
    <template v-if="compact === false && collapsed === false">
      <span
        v-for="direction in Object.values(ResizeDirection)"
        :key="direction"
        class="surface-resize"
        :data-overlay-resize="direction"
        aria-hidden="true"
      />
    </template>
  </section>
</template>
<style scoped>
.floating-surface {
  position: fixed;
  display: flex;
  flex-direction: column;
  pointer-events: auto;
  z-index: 10000;
  background: var(--prism-panel-background);
  border: 1px solid var(--prism-panel-border);
  border-radius: var(--prism-radius-card);
  box-shadow: 0 8px 28px #0005;
  min-height: 0;
}
.surface-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
  min-height: 44px;
  gap: 6px;
  padding: 0 6px 0 12px;
  zoom: var(--shell-ui-scale, 1);
  cursor: move;
  user-select: none;
  background: var(--prism-panel-header);
  border-bottom: 1px solid var(--prism-panel-border);
  touch-action: none;
}
.surface-header strong {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--prism-text);
  font-size: 13px;
  font-weight: 600;
  margin-right: auto;
}
.surface-actions {
  display: flex;
  align-items: center;
  gap: 2px;
}
.surface-actions :deep(.ui-icon-button) {
  width: 24px;
  height: 24px;
}
.surface-content {
  flex: 1;
  min-height: 0;
  min-width: 0;
  display: flex;
  zoom: var(--shell-ui-scale, 1);
}
.surface-attachment {
  position: absolute;
  display: flex;
  left: calc(100% + 13px);
  top: -1px;
  bottom: -1px;
  width: calc(var(--surface-attachment-space) - 12px);
  min-width: 0;
  min-height: 0;
}
.surface-resize {
  position: absolute;
  z-index: 2;
  touch-action: none;
  user-select: none;
}

.surface-resize[data-overlay-resize='n'],
.surface-resize[data-overlay-resize='s'] {
  right: 10px;
  left: 10px;
  height: 5px;
  cursor: ns-resize;
}

.surface-resize[data-overlay-resize='e'],
.surface-resize[data-overlay-resize='w'] {
  top: 10px;
  bottom: 10px;
  width: 5px;
  cursor: ew-resize;
}

.surface-resize[data-overlay-resize='ne'],
.surface-resize[data-overlay-resize='nw'],
.surface-resize[data-overlay-resize='se'],
.surface-resize[data-overlay-resize='sw'] {
  width: 10px;
  height: 10px;
}

.surface-resize[data-overlay-resize='n'],
.surface-resize[data-overlay-resize='ne'],
.surface-resize[data-overlay-resize='nw'] {
  top: 0;
}

.surface-resize[data-overlay-resize='s'],
.surface-resize[data-overlay-resize='se'],
.surface-resize[data-overlay-resize='sw'] {
  bottom: 0;
}

.surface-resize[data-overlay-resize='e'],
.surface-resize[data-overlay-resize='ne'],
.surface-resize[data-overlay-resize='se'] {
  right: 0;
}

.surface-resize[data-overlay-resize='w'],
.surface-resize[data-overlay-resize='nw'],
.surface-resize[data-overlay-resize='sw'] {
  left: 0;
}

.surface-resize[data-overlay-resize='ne'],
.surface-resize[data-overlay-resize='sw'],
.floating-surface[data-resizing='ne'],
.floating-surface[data-resizing='sw'] {
  cursor: nesw-resize;
}

.surface-resize[data-overlay-resize='nw'],
.surface-resize[data-overlay-resize='se'],
.floating-surface[data-resizing='nw'],
.floating-surface[data-resizing='se'] {
  cursor: nwse-resize;
}

.floating-surface[data-resizing='n'],
.floating-surface[data-resizing='s'] {
  cursor: ns-resize;
}

.floating-surface[data-resizing='e'],
.floating-surface[data-resizing='w'] {
  cursor: ew-resize;
}

.surface-resize[data-overlay-resize='se']::after {
  position: absolute;
  right: 2px;
  bottom: 2px;
  width: 5px;
  height: 5px;
  border-right: 1px solid var(--prism-muted);
  border-bottom: 1px solid var(--prism-muted);
  content: '';
}

.floating-surface[data-resizing] * {
  cursor: inherit;
}

.surface-compact .surface-header {
  border-bottom: 0;
}
</style>
