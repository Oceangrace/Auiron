<script setup lang="ts">
import { inject, computed, ref, watch } from 'vue';
import { IconAssets } from '../../../renderer/assets/icon-assets';
import { iconAssetsContextKey } from '../../../renderer/assets/icon-assets-context-key';
const props = defineProps<{ icon: string }>();

const assets = inject(iconAssetsContextKey, () => new IconAssets(), true);

const symbols: Readonly<Record<string, string>> = {
  edit: '✎',
  settings: '⚙',
  chart: '▥',
  close: '×',
  panels: '▦',
};

const url = computed(() => (props.icon.startsWith('game:') ? assets.url(props.icon) : null));

const missing = ref(false);

watch(url, () => {
  missing.value = false;
});
</script>
<template>
  <span class="group-button-icon" aria-hidden="true">
    <img
      v-if="url !== null && !missing"
      :src="url"
      alt=""
      draggable="false"
      loading="lazy"
      @error="missing = true"
    />
    <svg
      v-else-if="icon === 'chevron'"
      viewBox="0 0 16 16"
      fill="none"
      stroke="currentColor"
      stroke-width="1.8"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M4 6 8 10 12 6" />
    </svg>
    <span v-else>{{ symbols[icon] ?? '◇' }}</span>
  </span>
</template>
<style scoped>
.group-button-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1em;
  height: 1em;
  flex-shrink: 0;
  line-height: 1;
  vertical-align: middle;
}
.group-button-icon img,
.group-button-icon svg {
  width: 100%;
  height: 100%;
  display: block;
}
.group-button-icon img {
  object-fit: contain;
  border-radius: 2px;
}
</style>
