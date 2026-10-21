<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { ShellController } from './shell-controller';
import { OverlayAnchor } from '../../core/preferences/overlay-anchor';
import { ToolbarLayout } from '../../core/preferences/toolbar-layout';
import FloatingSurface from './floating-surface.vue';
import UiIconButton from '../components/ui/ui-icon-button.vue';
import { ToolbarVisibility } from './toolbar-visibility';
const props = defineProps<{ shell: ShellController; editorOpen: boolean; settingsOpen: boolean }>();

const emit = defineEmits<{ editor: []; settings: []; quit: [] }>();

const preferences = props.shell.getState();

const visibility = new ToolbarVisibility();

const expanded = visibility.state();

const hidden = computed(() => preferences.value.toolbarAutoHide && expanded.value === false);

watch(
  () => preferences.value.toolbarAutoHide,
  () => {
    visibility.hide();
  },
);

const { t, locale } = useI18n();

const started = Date.now();

const now = ref(Date.now());

const clock = computed(() =>
  new Date(now.value).toLocaleTimeString(locale.value, { hour: '2-digit', minute: '2-digit' }),
);

const uptime = computed(() => {
  const minutes = Math.floor((now.value - started) / 60000);

  return String(Math.floor(minutes / 60)) + ':' + String(minutes % 60).padStart(2, '0');
});

const measured = ref({ width: 290, height: 50 });

let observer: ResizeObserver | null = null;

const viewport = ref({ width: window.innerWidth, height: window.innerHeight });

const refresh = (): void => {
  viewport.value = { width: window.innerWidth, height: window.innerHeight };
};

let timer: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
  const element = document.querySelector('.application-toolbar');

  if (element !== null) {
    observer = new ResizeObserver(() => {
      const rect = element.getBoundingClientRect();

      measured.value = { width: rect.width, height: rect.height };
    });
    observer.observe(element);
  }

  timer = setInterval(() => {
    now.value = Date.now();
  }, 1000);
  window.addEventListener('resize', refresh);
});
onBeforeUnmount(() => {
  visibility.dispose();
  observer?.disconnect();

  if (timer !== null) {
    clearInterval(timer);
  }

  window.removeEventListener('resize', refresh);
});
const position = computed(() => {
  if (preferences.value.toolbarAutoHide === true) {
    return { x: Math.max(0, (viewport.value.width - measured.value.width) / 2), y: 0 };
  }

  const anchor = preferences.value.toolbarAnchor;

  if (anchor === OverlayAnchor.Free && preferences.value.toolbarPosition !== null) {
    return {
      x: Math.max(
        0,
        Math.min(viewport.value.width - measured.value.width, preferences.value.toolbarPosition.x),
      ),
      y: Math.max(
        0,
        Math.min(
          viewport.value.height - measured.value.height,
          preferences.value.toolbarPosition.y,
        ),
      ),
    };
  }

  const width = measured.value.width;

  const height = measured.value.height;

  const coordinates = new Map([
    [OverlayAnchor.TopLeft, { x: 16, y: 12 }],
    [OverlayAnchor.TopCenter, { x: (viewport.value.width - width) / 2, y: 12 }],
    [OverlayAnchor.TopRight, { x: viewport.value.width - width - 16, y: 12 }],
    [OverlayAnchor.CenterLeft, { x: 16, y: (viewport.value.height - height) / 2 }],
    [
      OverlayAnchor.Center,
      { x: (viewport.value.width - width) / 2, y: (viewport.value.height - height) / 2 },
    ],
    [
      OverlayAnchor.CenterRight,
      { x: viewport.value.width - width - 16, y: (viewport.value.height - height) / 2 },
    ],
    [OverlayAnchor.BottomLeft, { x: 16, y: viewport.value.height - height - 16 }],
    [
      OverlayAnchor.BottomCenter,
      { x: (viewport.value.width - width) / 2, y: viewport.value.height - height - 16 },
    ],
    [
      OverlayAnchor.BottomRight,
      { x: viewport.value.width - width - 16, y: viewport.value.height - height - 16 },
    ],
  ]);

  return coordinates.get(anchor) ?? { x: 16, y: 12 };
});
</script>
<template>
  <FloatingSurface
    compact
    class="application-toolbar"
    :x="position.x"
    :y="position.y"
    :class="{
      'toolbar-vertical':
        !preferences.toolbarAutoHide && preferences.toolbarLayout === ToolbarLayout.Vertical,
      'toolbar-autohide': preferences.toolbarAutoHide,
      'toolbar-hidden': hidden,
    }"
    :draggable="!preferences.toolbarAutoHide"
    :interactive="!hidden"
    @pointerenter="visibility.reveal()"
    @pointerleave="visibility.leave()"
    @focusin="visibility.focus($event)"
    @focusout="visibility.blur($event)"
    @keydown.esc.stop="visibility.hide()"
    @moved="shell.update({ toolbarAnchor: OverlayAnchor.Free, toolbarPosition: $event })"
  >
    <template #header>
      <span v-if="preferences.toolbarShowClock" class="toolbar-time">{{ clock }}</span>
      <span
        v-if="preferences.toolbarShowPlayTime"
        class="toolbar-time"
        :title="t('shell.uptime')"
        >{{ uptime }}</span
      >
      <UiIconButton
        class="toolbar-icon"
        :label="t('modules.interfaces.title')"
        :active="editorOpen"
        :aria-pressed="editorOpen"
        @click="emit('editor')"
      >
        <svg viewBox="0 0 20 20" aria-hidden="true">
          <path d="M3 3h5v5H3z M12 3h5v5h-5z M3 12h5v5H3z M12 12h5v5h-5z" />
        </svg>
      </UiIconButton>
      <UiIconButton
        class="toolbar-icon"
        :label="t('shell.settings')"
        :active="settingsOpen"
        :aria-pressed="settingsOpen"
        @click="emit('settings')"
      >
        <svg viewBox="0 0 20 20" aria-hidden="true">
          <circle cx="10" cy="10" r="3" />
          <path d="M8 2h4l1 3 3-1 2 4-2 2 2 2-2 4-3-1-1 3H8l-1-3-3 1-2-4 2-2-2-2 2-4 3 1z" />
        </svg>
      </UiIconButton>
      <span class="toolbar-status"><slot name="status" /></span>
      <UiIconButton
        class="toolbar-icon"
        :label="t('shell.minimize')"
        @click="void shell.minimize()"
      >
        <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M5 10h10" /></svg>
      </UiIconButton>
      <UiIconButton class="toolbar-icon" :label="t('shell.quit')" @click="emit('quit')">
        <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m5 5 10 10 M15 5 5 15" /></svg>
      </UiIconButton>
    </template>
    <template v-if="preferences.toolbarAutoHide" #edge>
      <button
        type="button"
        class="toolbar-reveal"
        data-overlay-interactive
        :aria-label="t(hidden ? 'shell.toolbarShow' : 'shell.toolbarHide')"
        :aria-expanded="!hidden"
        @click="hidden ? visibility.reveal() : visibility.hide()"
      >
        <svg viewBox="0 0 20 20" aria-hidden="true">
          <path :d="hidden ? 'm5 7 5 5 5-5' : 'm5 12 5-5 5 5'" />
        </svg>
      </button>
    </template>
  </FloatingSurface>
</template>
<style scoped>
.application-toolbar {
  z-index: 10010;
}
.toolbar-autohide {
  max-width: calc(100vw - 16px);
  border-radius: 0 0 var(--prism-radius-card) var(--prism-radius-card);
  transition: transform 180ms ease-out;
}
.toolbar-autohide :deep(.surface-header) {
  cursor: default;
  flex-wrap: wrap;
}
.toolbar-hidden {
  transform: translateY(-100%);
  pointer-events: none;
  box-shadow: none;
}
.toolbar-reveal {
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  display: grid;
  place-items: center;
  width: 44px;
  height: 22px;
  padding: 0;
  pointer-events: auto;
  cursor: pointer;
  color: var(--prism-accent);
  background: var(--prism-panel-background);
  border: 1px solid var(--prism-panel-border);
  border-top: 0;
  border-radius: 0 0 8px 8px;
  zoom: var(--shell-ui-scale, 1);
}
.toolbar-reveal svg {
  width: 16px;
  height: 16px;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.5;
}
.toolbar-reveal:hover,
.toolbar-reveal:focus-visible {
  color: var(--prism-text);
  outline: 1px solid var(--prism-accent);
  outline-offset: -2px;
}
@media (prefers-reduced-motion: reduce) {
  .toolbar-autohide {
    transition: none;
  }
}

.toolbar-status {
  display: flex;
  align-items: center;
  margin-left: 4px;
  padding-left: 8px;
  border-left: 1px solid var(--prism-border-dim);
}
.toolbar-time {
  padding: 0 8px;
  font-size: 12px;
  font-variant-numeric: tabular-nums;
  min-width: 40px;
  color: var(--prism-label);
}
.application-toolbar :deep(.surface-header) {
  min-height: 0;
  gap: 4px;
  padding: 5px;
}
.application-toolbar :deep(.toolbar-icon) {
  width: 30px;
  height: 30px;
}
.application-toolbar :deep(.toolbar-icon svg) {
  width: 16px;
  height: 16px;
}
.toolbar-vertical .toolbar-status {
  margin-left: 0;
  padding: 6px 0 0;
  border-left: 0;
  border-top: 1px solid var(--prism-border-dim);
}
.toolbar-vertical .toolbar-time {
  padding: 4px 2px;
  text-align: center;
}
.toolbar-vertical :deep(.surface-header) {
  flex-direction: column;
}
</style>
