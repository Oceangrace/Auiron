<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { AppearancePreview } from './appearance-preview';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
const props = defineProps<{ preview: AppearancePreview; widgetId: string }>();

const { t } = useI18n();

const context = props.preview.context;

const entries = computed(() => Object.entries(context.value.fields ?? {}));

const field = (key: string, value: string): void => {
  let parsed: string | number | boolean | null = value;

  if (value === 'true') {
    parsed = true;
  } else if (value === 'false') {
    parsed = false;
  } else if (value === '?') {
    parsed = null;
  } else if (value.trim() !== '' && Number.isFinite(Number(value)) === true) {
    parsed = Number(value);
  }

  props.preview.update({ fields: { ...context.value.fields, [key]: parsed } });
};

const rename = (before: string, after: string): void => {
  if (after === '') {
    return;
  }

  const fields = Object.fromEntries(
    Object.entries(context.value.fields ?? {}).filter(([key]) => key !== before),
  );

  fields[after] = context.value.fields?.[before] ?? null;
  props.preview.update({ fields });
};

const row = (index: number, key: string, value: string): void => {
  props.preview.update({
    rows: (context.value.rows ?? []).map((row, at) =>
      at === index ? { ...row, [key]: key === 'row.name' ? value : Number(value) } : row,
    ),
  });
};

const addMatch = (): void => {
  const instances = context.value.instances ?? {};

  props.preview.update({
    instances: {
      ...instances,
      [props.widgetId]: [
        ...(instances[props.widgetId] ?? []),
        {
          't1.name': t('modules.interfaces.previewFields.match'),
          't1.stacks': 1,
          't1.remaining': 5000,
        },
      ],
    },
  });
};
</script>
<template>
  <div class="preview-fields">
    <h3>{{ t('modules.interfaces.previewFields.fields') }}</h3>
    <div v-for="[key, value] in entries" :key="key" class="field-grid">
      <UiInput
        :model-value="key"
        :label="t('modules.interfaces.advanced.path')"
        @update:model-value="rename(key, $event)"
      />
      <UiInput
        :model-value="value === null ? '?' : String(value)"
        :label="t('modules.interfaces.conditions.value')"
        @update:model-value="field(key, $event)"
      />
      <UiButton
        @click="
          preview.update({ fields: Object.fromEntries(entries.filter(([name]) => name !== key)) })
        "
        >−</UiButton
      >
    </div>
    <UiButton
      class="is-add"
      @click="
        preview.update({
          fields: { ...context.fields, ['field' + String(entries.length + 1)]: null },
        })
      "
      >＋ {{ t('modules.interfaces.previewFields.field') }}</UiButton
    >
    <h3>{{ t('modules.interfaces.previewFields.rows') }}</h3>
    <div v-for="(item, index) in context.rows ?? []" :key="index" class="field-grid">
      <UiInput
        v-for="key in ['row.name', 'row.value', 'row.percent', 'row.leaderPercent']"
        :key="key"
        :model-value="String(item[key] ?? '')"
        :label="key"
        @update:model-value="row(index, key, $event)"
      />
      <UiButton
        @click="preview.update({ rows: context.rows?.filter((_, at) => at !== index) ?? [] })"
        >−</UiButton
      >
    </div>
    <UiButton
      class="is-add"
      @click="
        preview.update({
          rows: [
            ...(context.rows ?? []),
            {
              'row.name': t('modules.interfaces.previewFields.row'),
              'row.value': 100,
              'row.percent': 100,
              'row.leaderPercent': 100,
            },
          ],
        })
      "
      >＋ {{ t('modules.interfaces.previewFields.row') }}</UiButton
    >
    <h3>{{ t('modules.interfaces.previewFields.matches') }}</h3>
    <div
      v-for="(item, index) in context.instances?.[widgetId] ?? []"
      :key="index"
      class="field-grid"
    >
      <UiInput
        :model-value="String(item['t1.name'] ?? '')"
        :label="t('modules.interfaces.name')"
        @update:model-value="
          preview.update({
            instances: {
              ...context.instances,
              [widgetId]:
                context.instances?.[widgetId]?.map((value, at) =>
                  at === index ? { ...value, 't1.name': $event } : value,
                ) ?? [],
            },
          })
        "
      />
      <UiButton
        @click="
          preview.update({
            instances: {
              ...context.instances,
              [widgetId]: context.instances?.[widgetId]?.filter((_, at) => at !== index) ?? [],
            },
          })
        "
        >−</UiButton
      >
    </div>
    <UiButton class="is-add" @click="addMatch"
      >＋ {{ t('modules.interfaces.previewFields.match') }}</UiButton
    >
  </div>
</template>
<style scoped>
.preview-fields {
  display: grid;
  gap: 8px;
  grid-column: 1/-1;
  min-width: 0;
}
</style>
