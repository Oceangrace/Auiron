<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { ReleaseMonitor } from './release-monitor';
import UiButton from '../components/ui/ui-button.vue';
import UiIconButton from '../components/ui/ui-icon-button.vue';

const props = defineProps<{ releases: ReleaseMonitor }>();

const state = props.releases.getState();

const visible = computed(
  () =>
    state.value.info?.available === true &&
    state.value.info.latestVersion !== state.value.dismissedVersion,
);

const { t } = useI18n();
</script>
<template>
  <aside v-if="visible" class="release-notice" role="status">
    <span>{{ t('shell.updates.available', { version: state.info?.latestVersion }) }}</span>
    <UiButton @click="void releases.open()">{{ t('shell.updates.open') }}</UiButton>
    <UiIconButton :label="t('shell.updates.dismiss')" @click="releases.dismiss()">
      <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m5 5 10 10M15 5 5 15" /></svg>
    </UiIconButton>
    <p v-if="state.openFailed" role="alert">{{ t('shell.updates.openFailed') }}</p>
  </aside>
</template>
<style scoped>
.release-notice {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px;
  padding: 10px 16px;
  border-bottom: 1px solid var(--prism-panel-border);
  background: var(--prism-selected);
  font-size: 12px;
}
.release-notice > span {
  flex: 1;
  color: var(--prism-accent);
}
.release-notice > p {
  flex-basis: 100%;
  margin: 0;
  color: var(--prism-muted);
}
</style>
