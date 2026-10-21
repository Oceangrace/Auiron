<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { JournalNumberTransition } from './journal-number-transition';
const props = defineProps<{
  value: number;
  duration: number;
  resetKey: string;
  compact: boolean;
  decimals: number;
  partial?: boolean;
}>();

const { locale } = useI18n();

const current = ref<number | null>(props.value);

const transition = new JournalNumberTransition((value) => {
  current.value = value;
});

watch(
  [() => props.value, () => props.duration, () => props.resetKey],
  ([value, duration, key], previous) => {
    transition.update(value, previous[2] !== key ? 0 : duration);
  },
  { immediate: true },
);

const text = computed(() =>
  new Intl.NumberFormat(locale.value, {
    notation: props.compact ? 'compact' : 'standard',
    maximumFractionDigits: props.decimals,
  }).format(current.value ?? 0),
);

onUnmounted(() => {
  transition.dispose();
});
</script>
<template>
  <span>{{ partial ? '≥ ' : '' }}{{ text }}</span>
</template>
