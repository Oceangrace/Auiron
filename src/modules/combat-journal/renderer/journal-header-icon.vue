<script setup lang="ts">
import { JournalHeaderButton } from '../model/journal-header-button';
defineProps<{ button: JournalHeaderButton }>();

const paths: Readonly<Record<JournalHeaderButton, string>> = {
  [JournalHeaderButton.Metric]: 'M4 16V9 M10 16V4 M16 16V7',
  [JournalHeaderButton.Segment]: 'M10 5v5l3 2 M10 2a8 8 0 1 0 0 16 8 8 0 0 0 0-16',
  [JournalHeaderButton.NewWindow]: 'M4 10h12 M10 4v12',
  [JournalHeaderButton.Preview]:
    'M2 10s3-5 8-5 8 5 8 5-3 5-8 5-8-5Z M10 8a2 2 0 1 0 0 4 2 2 0 0 0 0-4',
  [JournalHeaderButton.NewRun]: 'M15 6a6 6 0 1 0 1 6 M15 2v4h-4',
  [JournalHeaderButton.Settings]:
    'm8 2 4 0 1 3 3 1 2 4-2 4-3 1-1 3H8l-1-3-3-1-2-4 2-4 3-1Z M10 7a3 3 0 1 0 0 6 3 3 0 0 0 0-6',
  [JournalHeaderButton.Close]: 'm5 5 10 10 M15 5 5 15',
};
</script>
<template>
  <svg viewBox="0 0 20 20" aria-hidden="true"><path :d="paths[button]" /></svg>
</template>
