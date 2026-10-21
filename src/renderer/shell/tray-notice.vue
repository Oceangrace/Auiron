<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import AurionMark from './aurion-mark.vue';
import UiIconButton from '../components/ui/ui-icon-button.vue';
import { TrayNoticeTimer } from './tray-notice-timer';

const emit = defineEmits<{ editor: []; guide: []; dismiss: [] }>();

const { t } = useI18n();

const hovered = ref(false);

const focused = ref(false);

const paused = computed(() => hovered.value || focused.value);

const timer = new TrayNoticeTimer(() => {
  emit('dismiss');
});

watch(paused, (value) => {
  if (value === true) {
    timer.pause();
  } else {
    timer.resume();
  }
});

onMounted(() => {
  timer.resume();
});

onBeforeUnmount(() => {
  timer.dispose();
});
</script>

<template>
  <aside
    class="tray-notice"
    data-overlay-interactive
    aria-labelledby="tray-notice-title"
    :style="{ '--notice-duration': String(TrayNoticeTimer.durationMs) + 'ms' }"
    :class="{ 'is-paused': paused }"
    @mouseenter="hovered = true"
    @mouseleave="hovered = false"
    @focusin="focused = true"
    @focusout="focused = false"
    @keydown.esc="emit('dismiss')"
  >
    <div class="tray-notice-icon"><AurionMark /></div>
    <div class="tray-notice-content">
      <div role="status">
        <h2 id="tray-notice-title">{{ t('shell.welcome.noticeTitle') }}</h2>
        <p>{{ t('shell.welcome.noticeDescription') }}</p>
      </div>
      <div class="tray-notice-actions">
        <button type="button" class="notice-editor" @click="emit('editor')">
          {{ t('shell.welcome.editor') }}
        </button>
        <button type="button" @click="emit('guide')">{{ t('shell.welcome.guide') }}</button>
      </div>
    </div>
    <UiIconButton
      class="tray-notice-close"
      :label="t('shell.welcome.closeNotice')"
      @click="emit('dismiss')"
    >
      <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m5 5 10 10M15 5 5 15" /></svg>
    </UiIconButton>
    <div class="tray-notice-progress" aria-hidden="true" />
  </aside>
</template>

<style scoped>
.tray-notice {
  position: fixed;
  right: 24px;
  bottom: 64px;
  display: flex;
  align-items: flex-start;
  gap: 10px;
  width: min(384px, calc(100vw - 32px));
  max-height: calc(100vh - 96px);
  overflow: auto;
  padding: 14px 28px 14px 14px;
  border: 1px solid #ffffff16;
  border-radius: 12px;
  background: var(--prism-background);
  box-shadow: 0 12px 48px #0004;
  pointer-events: auto;
}
.tray-notice-icon {
  display: grid;
  flex-shrink: 0;
  place-items: center;
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: #42bbb90b;
}
.tray-notice-icon svg {
  width: 26px;
  height: 26px;
}
.tray-notice-content {
  min-width: 0;
}
.tray-notice h2 {
  margin: 0 0 4px;
  font-size: 14px;
  font-weight: 650;
  letter-spacing: -0.3px;
}
.tray-notice p {
  margin: 0;
  color: var(--prism-muted);
  font-size: 12px;
  line-height: 1.45;
}
.tray-notice-close {
  position: absolute;
  top: 6px;
  right: 6px;
}
.tray-notice-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px 16px;
  margin-top: 10px;
}
.tray-notice-actions button {
  padding: 0;
  border: 0;
  background: transparent;
  color: var(--prism-dim);
  font-size: 12px;
}
.tray-notice-actions .notice-editor {
  color: var(--prism-accent);
  font-weight: 600;
}
.tray-notice-actions button:hover {
  color: var(--prism-text);
}
.tray-notice-progress {
  position: absolute;
  inset: auto 0 0;
  height: 2px;
  background: var(--prism-accent);
  transform-origin: left;
  animation: notice-countdown var(--notice-duration) linear forwards;
}
.is-paused .tray-notice-progress {
  animation-play-state: paused;
}
@keyframes notice-countdown {
  from {
    transform: scaleX(1);
  }
  to {
    transform: scaleX(0);
  }
}
@media (prefers-reduced-motion: reduce) {
  .tray-notice-progress {
    animation: none;
    display: none;
  }
}
@media (max-width: 420px) {
  .tray-notice {
    right: 16px;
  }
}
</style>
