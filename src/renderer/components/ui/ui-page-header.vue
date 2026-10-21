<script setup lang="ts">
withDefaults(defineProps<{ title: string; subtitle?: string }>(), { subtitle: '' });
</script>

<template>
  <header class="ui-page-header">
    <div v-if="$slots['thumbnail']" class="ui-page-thumbnail"><slot name="thumbnail" /></div>
    <div class="ui-page-title">
      <div class="ui-page-breadcrumb">
        <slot name="breadcrumb" />
        <h2>{{ title }}</h2>
      </div>
      <p v-if="subtitle !== ''">{{ subtitle }}</p>
      <slot name="meta" />
    </div>
    <div class="ui-page-actions"><slot name="actions" /></div>
  </header>
</template>

<style scoped>
.ui-page-header {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 18px 24px;
  min-height: 4rem;
  flex-shrink: 0;
  border-bottom: 1px solid var(--prism-border);
}
.ui-page-thumbnail {
  display: grid;
  place-items: center;
  width: 32px;
  height: 32px;
  flex-shrink: 0;
  color: var(--prism-muted);
}
.ui-page-thumbnail :deep(svg) {
  width: 24px;
  height: 24px;
}
.ui-page-title {
  min-width: 0;
  flex: 1;
}
.ui-page-breadcrumb {
  display: flex;
  min-width: 0;
  align-items: center;
  gap: 0.46rem;
}
.ui-page-header h2 {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin: 0;
  font-size: 15px;
  font-weight: 600;
  line-height: 1.4;
}
.ui-page-title > p {
  margin: 0.154rem 0 0;
  color: var(--prism-dim);
  font-size: 0.923rem;
  line-height: 1.4;
}
.ui-page-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
  max-width: 62%;
}
</style>
