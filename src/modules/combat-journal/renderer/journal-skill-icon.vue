<script setup lang="ts">
import { computed, inject, ref, watch } from 'vue';
import { IconAssets } from '../../../renderer/assets/icon-assets';
import { iconAssetsContextKey } from '../../../renderer/assets/icon-assets-context-key';

const props = defineProps<{ path: string | null }>();

const assets = inject(iconAssetsContextKey, () => new IconAssets(), true);

const url = computed(() => assets.url(props.path));

const failed = ref(false);

watch(url, () => {
  failed.value = false;
});
</script>
<template>
  <span class="journal-skill-icon" aria-hidden="true">
    <img v-if="url && !failed" :src="url" alt="" loading="lazy" @error="failed = true" />
    <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.4">
      <path d="m13 3-8 11h6l-1 7 9-12h-6z" />
    </svg>
  </span>
</template>
<style scoped>
.journal-skill-icon {
  display: inline-flex;
  width: 26px;
  height: 26px;
  flex: 0 0 26px;
  vertical-align: middle;
  overflow: hidden;
  border-radius: 4px;
  background: var(--prism-field);
  border: 1px solid var(--prism-border);
  color: var(--prism-muted);
}
.journal-skill-icon img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.journal-skill-icon svg {
  margin: 4px;
  width: 16px;
  height: 16px;
}
</style>
