<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { ReleaseMonitor } from './release-monitor';
import UiButton from '../components/ui/ui-button.vue';

const props = defineProps<{ releases: ReleaseMonitor }>();

const { t } = useI18n();

const state = props.releases.getState();
</script>
<template>
  <section class="release-settings">
    <div class="release-version">
      <strong>{{ t('shell.updates.version', { version: state.currentVersion ?? '—' }) }}</strong>
      <span v-if="state.info?.available === true" class="release-available" role="status">{{
        t('shell.updates.available', { version: state.info.latestVersion })
      }}</span>
      <span
        v-else-if="state.info?.latestVersion != null && state.checkFailed === false"
        role="status"
        >{{ t('shell.updates.current') }}</span
      >
    </div>
    <p>{{ t('shell.updates.hint') }}</p>
    <div class="release-actions">
      <UiButton :disabled="state.checking" @click="void releases.check()">{{
        t(state.checking ? 'shell.updates.checking' : 'shell.updates.check')
      }}</UiButton>
      <UiButton
        :class="{ 'is-primary': state.info?.available === true }"
        @click="void releases.open()"
        >{{ t('shell.updates.open') }}</UiButton
      >
    </div>
    <p v-if="state.checkFailed" role="status">{{ t('shell.updates.checkFailed') }}</p>
    <p v-if="state.openFailed" role="alert">{{ t('shell.updates.openFailed') }}</p>
  </section>
</template>
<style scoped>
.release-settings,
.release-version {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.release-settings p,
.release-version > span {
  margin: 0;
  color: var(--prism-muted);
  font-size: 12px;
  line-height: 1.5;
}
.release-version strong {
  font-size: 14px;
  font-weight: 600;
}
.release-version > .release-available {
  color: var(--prism-accent);
}
.release-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
</style>
