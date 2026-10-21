<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, reactive, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { JournalHeaderButton } from '../model/journal-header-button';
import { JournalHeaderOrder } from './journal-header-order';
import JournalHeaderIcon from './journal-header-icon.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import UiIconButton from '../../../renderer/components/ui/ui-icon-button.vue';
const props = defineProps<{ buttons: readonly JournalHeaderButton[] }>();

const emit = defineEmits<{ change: [value: readonly JournalHeaderButton[]] }>();

const { t } = useI18n();

const order = reactive(
  new JournalHeaderOrder(
    () => props.buttons,
    (buttons) => {
      emit('change', buttons);
    },
  ),
);

const entries = computed(() => order.draft ?? props.buttons);

const selectedIndex = computed(() =>
  order.selected === null ? -1 : props.buttons.indexOf(order.selected),
);

const toggle = (button: JournalHeaderButton, enabled: boolean): void => {
  order.cancel();
  emit(
    'change',
    enabled ? [...props.buttons, button] : props.buttons.filter((entry) => entry !== button),
  );
};

const cancel = (): void => {
  order.cancel();
};

const keydown = (event: KeyboardEvent): void => {
  if (event.key === 'Escape') {
    cancel();
  }
};

const moveKey = (event: KeyboardEvent, button: JournalHeaderButton, offset: number): void => {
  const element = event.currentTarget;

  order.move(button, offset);
  void nextTick(() => {
    if (element instanceof HTMLElement === true) {
      element.focus({ preventScroll: true });
    }
  });
};

watch(() => props.buttons, cancel);

onMounted(() => {
  window.addEventListener('blur', cancel);
  window.addEventListener('keydown', keydown);
});
onUnmounted(() => {
  cancel();
  window.removeEventListener('blur', cancel);
  window.removeEventListener('keydown', keydown);
});
</script>
<template>
  <div class="journal-header-editor">
    <p class="journal-order-hint">{{ t('modules.combatJournal.headerOrderHint') }}</p>
    <div
      class="journal-order-strip"
      :class="{ 'is-dragging': order.dragging }"
      role="group"
      :aria-label="t('modules.combatJournal.headerButtonList')"
      @pointermove="order.drag($event)"
      @pointerup="order.finish()"
      @pointercancel="cancel"
      @lostpointercapture="cancel"
    >
      <template v-for="button in entries" :key="button">
        <button
          type="button"
          class="journal-order-item"
          :data-header-button="button"
          :aria-label="t('modules.combatJournal.headerButtons.' + button)"
          :title="t('modules.combatJournal.headerButtons.' + button)"
          :aria-pressed="order.selected === button"
          @click="order.selected = button"
          @pointerdown="order.begin($event, button)"
          @keydown.left.prevent="moveKey($event, button, -1)"
          @keydown.right.prevent="moveKey($event, button, 1)"
          @keydown.esc.prevent="cancel"
        >
          <span v-if="button === JournalHeaderButton.Metric" class="journal-order-text">DPS</span>
          <span v-else-if="button === JournalHeaderButton.Segment" class="journal-order-text"
            >0:08</span
          >
          <JournalHeaderIcon v-else :button="button" />
        </button>
        <span
          v-if="button === JournalHeaderButton.Metric"
          class="journal-order-space"
          aria-hidden="true"
        />
      </template>
      <span v-if="entries.length === 0" class="journal-order-hint">{{
        t('modules.combatJournal.headerOrderEmpty')
      }}</span>
    </div>
    <div class="journal-order-selection">
      <span aria-live="polite">{{
        selectedIndex === -1
          ? t('modules.combatJournal.headerOrderSelect')
          : t('modules.combatJournal.headerButtons.' + order.selected)
      }}</span>
      <UiIconButton
        :label="t('modules.combatJournal.moveLeft')"
        :disabled="selectedIndex <= 0"
        @click="order.selected !== null && order.move(order.selected, -1)"
      >
        <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m12 5-5 5 5 5" /></svg>
      </UiIconButton>
      <UiIconButton
        :label="t('modules.combatJournal.moveRight')"
        :disabled="selectedIndex === -1 || selectedIndex === buttons.length - 1"
        @click="order.selected !== null && order.move(order.selected, 1)"
      >
        <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m8 5 5 5-5 5" /></svg>
      </UiIconButton>
    </div>
    <div class="journal-header-visibility">
      <UiSwitch
        v-for="button in Object.values(JournalHeaderButton)"
        :key="button"
        :model-value="buttons.includes(button)"
        :label="t('modules.combatJournal.headerButtons.' + button)"
        @update:model-value="toggle(button, $event)"
      />
    </div>
  </div>
</template>
<style scoped>
.journal-header-editor {
  display: grid;
  gap: 10px;
  min-width: 0;
}
.journal-order-hint {
  margin: 0;
  color: var(--prism-muted);
  font-size: 12px;
  line-height: 1.5;
}
.journal-order-strip {
  display: flex;
  align-items: center;
  gap: 4px;
  min-height: 52px;
  padding: 8px;
  overflow-x: auto;
  border: 1px solid var(--prism-border);
  border-radius: var(--prism-radius-control);
  background: var(--prism-card);
}
.journal-order-item {
  display: grid;
  place-items: center;
  flex: 0 0 auto;
  min-width: 34px;
  height: 34px;
  padding: 0 8px;
  border: 1px solid transparent;
  background: var(--prism-field);
  color: var(--prism-text);
  cursor: grab;
  touch-action: none;
  user-select: none;
}
.journal-order-item:hover {
  background: var(--prism-raised);
}
.journal-order-item[aria-pressed='true'] {
  border-color: var(--prism-accent);
  background: var(--prism-selected);
  color: var(--prism-accent);
}
.is-dragging .journal-order-item {
  cursor: grabbing;
}
.journal-order-item svg {
  width: 18px;
  height: 18px;
  fill: none;
  stroke: currentColor;
  stroke-width: 1.5;
  stroke-linecap: round;
  stroke-linejoin: round;
}
.journal-order-text {
  font-variant-numeric: tabular-nums;
}
.journal-order-space {
  flex: 1;
  min-width: 12px;
}
.journal-order-selection {
  display: flex;
  align-items: center;
  gap: 6px;
  min-height: 28px;
}
.journal-order-selection > span {
  flex: 1;
  min-width: 0;
  color: var(--prism-muted);
}
.journal-order-selection :deep(.ui-icon-button) {
  border: 1px solid var(--prism-border);
  background: var(--prism-field);
}
.journal-header-visibility {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 8px 16px;
  padding-top: 10px;
  border-top: 1px solid var(--prism-border-dim);
}
.journal-header-visibility :deep(.ui-switch) {
  flex-direction: row-reverse;
  justify-content: space-between;
  min-height: 36px;
  padding: 6px 10px;
  border: 1px solid var(--prism-border);
  border-radius: 5px;
  background: var(--prism-field);
}
</style>
