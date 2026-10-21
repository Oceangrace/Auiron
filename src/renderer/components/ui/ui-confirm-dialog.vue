<script setup lang="ts">
import { ref, useId, watch } from 'vue';
import UiButton from './ui-button.vue';

const props = defineProps<{
  open: boolean;
  title: string;
  message: string;
  cancelLabel: string;
  confirmLabel: string;
}>();

const emit = defineEmits<{ cancel: []; confirm: [] }>();

const dialog = ref<HTMLDialogElement | null>(null);

const id = useId();

watch(
  [() => props.open, dialog],
  () => {
    if (props.open === true && dialog.value?.open === false) {
      dialog.value.showModal();
    }

    if (props.open === false && dialog.value?.open === true) {
      dialog.value.close();
    }
  },
  { flush: 'post' },
);
</script>
<template>
  <dialog
    ref="dialog"
    class="ui-confirm-dialog"
    :aria-labelledby="id + '-title'"
    :aria-describedby="id + '-message'"
    data-overlay-interactive
    @cancel.prevent="emit('cancel')"
    @keydown.stop
  >
    <h2 :id="id + '-title'">{{ title }}</h2>
    <p :id="id + '-message'">{{ message }}</p>
    <div class="confirm-actions">
      <UiButton autofocus @click="emit('cancel')">{{ cancelLabel }}</UiButton>
      <UiButton class="confirm-danger" @click="emit('confirm')">{{ confirmLabel }}</UiButton>
    </div>
  </dialog>
</template>
<style scoped>
.ui-confirm-dialog {
  width: min(430px, calc(100vw - 40px));
  padding: 24px;
  border: 1px solid var(--prism-border);
  border-radius: var(--prism-radius-popup);
  background: #202829;
  color: var(--prism-text);
  box-shadow: 0 20px 70px #0008;
}
.ui-confirm-dialog::backdrop {
  background: #080d10aa;
}
h2 {
  margin: 0 0 12px;
  font-size: 17px;
  overflow-wrap: anywhere;
}
p {
  margin: 0;
  color: var(--prism-muted);
  line-height: 1.65;
}
.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 24px;
}
.confirm-danger {
  color: #fff;
  border-color: #b85855;
  background: #a84845;
}
</style>
