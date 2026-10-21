<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import UiButton from '../components/ui/ui-button.vue';
import UiIconButton from '../components/ui/ui-icon-button.vue';
import UiMessage from '../components/ui/ui-message.vue';
import AurionMark from './aurion-mark.vue';

defineProps<{ error: string | null; showGuide?: boolean }>();

const emit = defineEmits<{ editor: []; dismiss: [] }>();

const { t } = useI18n();
</script>

<template>
  <section
    class="welcome-screen"
    :class="{ 'has-guide': showGuide }"
    data-overlay-interactive
    role="dialog"
    aria-labelledby="welcome-title"
    aria-describedby="welcome-description"
    @keydown.esc="emit('dismiss')"
  >
    <div class="welcome-art">
      <AurionMark class="welcome-watermark" viewBox="8 21 112 86" />
    </div>
    <header class="welcome-brand">
      <AurionMark class="welcome-logo" />
      <span>Aurion</span>
      <UiIconButton class="welcome-close" :label="t('shell.welcome.hide')" @click="emit('dismiss')">
        <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m5 5 10 10M15 5 5 15" /></svg>
      </UiIconButton>
    </header>

    <div class="welcome-body">
      <p class="welcome-status" role="status"><span />{{ t('shell.welcome.searching') }}</p>
      <h1 id="welcome-title">{{ t('shell.welcome.title') }}</h1>
      <p id="welcome-description">{{ t('shell.welcome.description') }}</p>
      <UiMessage v-if="error !== null" :text="error" />
    </div>

    <div class="welcome-actions">
      <UiButton class="welcome-editor is-primary" @click="emit('editor')">
        {{ t('shell.welcome.editor') }}
        <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M4 10h12m-5-5 5 5-5 5" /></svg>
      </UiButton>
    </div>

    <div v-if="showGuide" id="welcome-guide" class="welcome-guide">
      <ol>
        <li>{{ t('shell.welcome.steps.profile') }}</li>
        <li>{{ t('shell.welcome.steps.game') }}</li>
        <li>{{ t('shell.welcome.steps.journal') }}</li>
      </ol>
      <p>{{ t('shell.welcome.trayHint') }}</p>
    </div>
  </section>
</template>

<style scoped>
.welcome-screen {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: flex;
  flex-direction: column;
  width: min(460px, calc(100vw - 32px));
  max-height: calc(100vh - 32px);
  overflow: auto;
  isolation: isolate;
  padding: 28px 35px 32px;
  border: 1px solid #ffffff14;
  border-radius: 16px;
  background: var(--prism-background);
  box-shadow:
    0 24px 80px #0005,
    inset 0 1px #ffffff04;
  pointer-events: auto;
}
.welcome-art {
  position: absolute;
  inset: 0;
  z-index: -1;
  overflow: hidden;
  border-radius: inherit;
  pointer-events: none;
}
.welcome-watermark {
  position: absolute;
  top: 0;
  right: -150px;
  width: auto;
  height: 100%;
  aspect-ratio: 112 / 86;
  opacity: 0.035;
  pointer-events: none;
}
.welcome-brand {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
  font-size: 18px;
  font-weight: 600;
  letter-spacing: -0.5px;
}
.welcome-logo {
  width: 26px;
  height: 26px;
}
.welcome-close {
  margin-left: auto;
  color: var(--prism-dim);
}
.welcome-body {
  margin: 64px 0 70px;
  flex-shrink: 0;
}
.has-guide .welcome-body {
  margin: 40px 0;
}
.welcome-status {
  display: flex;
  align-items: center;
  gap: 10px;
  margin: 0 0 20px;
  color: var(--prism-accent);
  font-size: 14px;
  font-weight: 500;
}
.welcome-status span {
  width: 20px;
  height: 20px;
  border: 2px solid #42bbb92b;
  border-top-color: var(--prism-accent);
  border-right-color: var(--prism-accent);
  border-radius: 50%;
  animation: welcome-search 1.6s linear infinite;
}
#welcome-title {
  margin: 0;
  font-size: clamp(26px, 5.8vw, 35px);
  line-height: 1.16;
  font-weight: 700;
  letter-spacing: -1.1px;
  white-space: pre-line;
}
#welcome-description {
  margin: 20px 0 0;
  color: var(--prism-muted);
  font-size: 13px;
  line-height: 1.6;
}
.welcome-actions {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}
.welcome-editor {
  min-height: 36px;
  padding: 0 16px;
  gap: 8px;
  border-radius: 7px;
  font-size: 13px;
  font-weight: 600;
}
.welcome-editor svg {
  width: 16px;
  height: 16px;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.7;
  stroke-linecap: round;
  stroke-linejoin: round;
}
.welcome-guide {
  flex-shrink: 0;
  margin-top: 26px;
  padding-top: 22px;
  border-top: 1px solid #ffffff0c;
  color: var(--prism-muted);
  font-size: 12px;
  line-height: 1.6;
}
.welcome-guide ol {
  display: grid;
  gap: 12px;
  margin: 0;
  padding-left: 16px;
}
.welcome-guide li {
  padding-left: 4px;
}
.welcome-guide li::marker {
  color: var(--prism-accent);
}
.welcome-guide p {
  margin: 16px 0 0;
  color: var(--prism-dim);
  font-size: 11px;
}
@keyframes welcome-search {
  to {
    transform: rotate(360deg);
  }
}
@media (prefers-reduced-motion: reduce) {
  .welcome-status span {
    animation: none;
  }
}
@media (max-width: 460px) {
  .welcome-screen {
    padding: 24px;
  }
}
@media (max-height: 680px) {
  .welcome-body {
    margin: 48px 0 56px;
  }
}
@media (max-height: 480px) {
  .welcome-body {
    margin: 30px 0;
  }
}
</style>
