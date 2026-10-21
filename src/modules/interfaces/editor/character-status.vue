<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { LiveFrame } from '../model/live-frame';
import { SourceStatus } from '../model/source-status';

const props = defineProps<{ frame: LiveFrame | null; error: string | null }>();

const waiting = computed(
  () =>
    props.frame?.status === SourceStatus.Live &&
    props.frame.characterDataComplete === false &&
    props.error === null,
);

const { t } = useI18n();
</script>
<template>
  <aside v-if="waiting" class="character-status" role="status">
    <strong>{{ t('modules.interfaces.runtime.waitingForCharacter') }}</strong>
    <p>{{ t('modules.interfaces.runtime.firstLaunchHint') }}</p>
  </aside>
</template>
<style scoped>
.character-status {
  padding: 12px 20px;
  border-bottom: 1px solid var(--prism-panel-border);
  background: #e6b9780d;
  font-size: 12px;
  line-height: 1.5;
}
.character-status strong {
  color: #e6b978;
  font-weight: 600;
}
.character-status p {
  margin: 4px 0 0;
  color: var(--prism-muted);
}
</style>
