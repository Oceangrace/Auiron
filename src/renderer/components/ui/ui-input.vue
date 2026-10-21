<script setup lang="ts">
import { ref, watch } from 'vue';
const props = defineProps<{ modelValue: string; label: string; multiline?: boolean }>();

const emit = defineEmits<{ 'update:modelValue': [value: string] }>();

const draft = ref(props.modelValue);

const focused = ref(false);

const committed = ref(props.modelValue);

watch(
  () => props.modelValue,
  (value) => {
    if (focused.value === false) {
      draft.value = value;
      committed.value = value;
    }
  },
);

const commit = (): void => {
  focused.value = false;

  if (draft.value === committed.value) {
    return;
  }

  committed.value = draft.value;
  emit('update:modelValue', draft.value);
};
</script>
<template>
  <textarea
    v-if="multiline"
    v-model="draft"
    class="ui-input ui-textarea"
    :aria-label="label"
    rows="3"
    maxlength="2048"
    @focus="focused = true"
    @change="commit"
    @blur="commit"
  />
  <input
    v-else
    v-model="draft"
    class="ui-input"
    :aria-label="label"
    maxlength="128"
    @focus="focused = true"
    @change="commit"
    @blur="commit"
  />
</template>
