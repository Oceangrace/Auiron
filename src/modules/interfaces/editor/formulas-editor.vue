<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import type { InterfaceDocument } from '../model/interface-document';
import type { FormulaDefinition } from '../model/rules/formula-definition';
import { DataKind } from '../model/rules/data-kind';
import { FormulaNodeKind } from '../model/rules/formula-node-kind';
import type { EditorSession } from './editor-session';
import FormulaNodeEditor from './formula-node-editor.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
const props = defineProps<{ document: InterfaceDocument; session: EditorSession }>();

const { t } = useI18n();

const state = props.session.getState();

const selectedId = ref<string | null>(null);

const copiedId = ref<string | null>(null);

const selected = computed(
  () =>
    props.document.formulas.find((formula) => formula.id === selectedId.value) ??
    props.document.formulas[0] ??
    null,
);

const kinds = [DataKind.Number, DataKind.Boolean, DataKind.Percent, DataKind.Duration] as const;

const add = (): void => {
  const id = crypto.randomUUID();

  update([
    ...props.document.formulas,
    {
      id,
      name: t('modules.interfaces.formulas.new'),
      kind: DataKind.Number,
      expression: { kind: FormulaNodeKind.Constant, value: 0 },
    },
  ]);
  selectedId.value = id;
};

const update = (formulas: readonly FormulaDefinition[]): void => {
  props.session.setFormulas(formulas);
};

const patch = (change: Partial<FormulaDefinition>): void => {
  update(
    props.document.formulas.map((value) =>
      value.id === selected.value?.id ? { ...value, ...change } : value,
    ),
  );
};

const copyReference = async (): Promise<void> => {
  const formula = selected.value;

  if (formula === null) {
    return;
  }

  try {
    await navigator.clipboard.writeText('{formula.' + formula.id + '}');
    copiedId.value = formula.id;
  } catch (error) {
    props.session.report(error);
  }
};
</script>
<template>
  <section class="formulas-editor editor-page">
    <header class="formulas-heading">
      <h3>
        {{ t('modules.interfaces.formulas.title') }} <small>{{ document.formulas.length }}</small>
      </h3>
      <UiButton
        class="is-add"
        :disabled="state.readOnly || document.formulas.length >= 32"
        @click="add()"
        >＋ {{ t('modules.interfaces.formulas.new') }}</UiButton
      >
    </header>
    <div v-if="selected !== null" class="formulas-workspace">
      <nav class="formula-list editor-card" :aria-label="t('modules.interfaces.formulas.title')">
        <button
          v-for="formula in document.formulas"
          :key="formula.id"
          type="button"
          :aria-pressed="selected.id === formula.id"
          @click="selectedId = formula.id"
        >
          <span class="formula-symbol" aria-hidden="true">f(x)</span>
          <span class="formula-list-label"
            ><strong>{{ formula.name }}</strong
            ><small>{{ t('modules.interfaces.formulas.types.' + formula.kind) }}</small></span
          >
        </button>
      </nav>
      <section :key="selected.id" class="formula-details" :aria-label="selected.name">
        <section class="editor-card">
          <h4>{{ t('modules.interfaces.information.main') }}</h4>
          <div class="formula-properties">
            <UiField :label="t('modules.interfaces.name')"
              ><UiInput
                :model-value="selected.name"
                :label="t('modules.interfaces.name')"
                :disabled="state.readOnly"
                @update:model-value="patch({ name: $event })"
            /></UiField>
            <UiField :label="t('modules.interfaces.formulas.type')">
              <UiSelect
                :model-value="selected.kind"
                :options="
                  kinds.map((value) => ({
                    value,
                    label: t('modules.interfaces.formulas.types.' + value),
                  }))
                "
                :label="t('modules.interfaces.formulas.type')"
                :disabled="state.readOnly"
                @update:model-value="patch({ kind: $event })"
              />
            </UiField>
          </div>
        </section>
        <section class="editor-card">
          <h4>{{ t('modules.interfaces.formulas.expression') }}</h4>
          <FormulaNodeEditor
            :node="selected.expression"
            :formulas="document.formulas.filter((value) => value.id !== selected?.id)"
            @change="patch({ expression: $event })"
          />
        </section>
        <div class="formula-reference editor-card">
          <section class="formula-text-reference">
            <h4>{{ t('modules.interfaces.formulas.textReference') }}</h4>
            <code>{{ '{formula.' + selected.id + '}' }}</code>
          </section>
          <UiButton class="formula-copy" @click="void copyReference()"
            ><svg viewBox="0 0 20 20" aria-hidden="true"><path d="M7 7h10v10H7z M3 13V3h10" /></svg
            ><span aria-live="polite">{{
              t(
                copiedId === selected.id
                  ? 'modules.interfaces.formulas.copied'
                  : 'modules.interfaces.formulas.copyReference',
              )
            }}</span></UiButton
          >
        </div>
        <UiButton
          class="formula-remove"
          :danger="true"
          :disabled="state.readOnly"
          @click="update(document.formulas.filter((formula) => formula.id !== selected?.id))"
          ><svg viewBox="0 0 20 20" aria-hidden="true">
            <path d="M4 6h12 M7 6V3h6v3 M5 6l1 11h8l1-11 M8 9v5 M12 9v5" /></svg
          >{{ t('modules.interfaces.formulas.remove') }}</UiButton
        >
      </section>
    </div>
    <p v-else class="formula-empty editor-card hint">
      {{ t('modules.interfaces.formulas.empty') }}
    </p>
  </section>
</template>
<style scoped>
.formulas-editor {
  overflow: hidden;
}
.formulas-heading {
  display: flex;
  flex-shrink: 0;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}
.formulas-heading h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}
.formulas-heading small {
  margin-left: 8px;
  color: var(--prism-muted);
  font-size: 12px;
  font-weight: 400;
}
.formulas-workspace {
  display: grid;
  grid-template-columns: minmax(130px, 180px) minmax(0, 1fr);
  gap: 20px;
  flex: 1;
  min-width: 0;
  min-height: 0;
}
.formula-list {
  gap: 4px;
  padding: 8px;
  overflow: auto;
  overscroll-behavior: contain;
}
.formula-list button {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  gap: 10px;
  width: 100%;
  min-width: 0;
  padding: 10px;
  border: 0;
  background: transparent;
  text-align: left;
}
.formula-list button:hover {
  background: var(--prism-raised);
}
.formula-list button[aria-pressed='true'] {
  background: var(--prism-selected);
  box-shadow: inset 2px 0 var(--prism-accent);
}
.formula-symbol {
  flex-shrink: 0;
  color: var(--prism-muted);
  font-size: 12px;
}
.formula-list-label {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}
.formula-list-label strong {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12px;
  font-weight: 500;
}
.formula-list-label small {
  color: var(--prism-muted);
  font-size: 11px;
}
.formula-details {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 20px;
  min-width: 0;
  min-height: 0;
  padding-right: 12px;
  overflow: auto;
  scrollbar-gutter: stable;
  overscroll-behavior: contain;
  container-type: inline-size;
}
.formula-details > .editor-card {
  flex-shrink: 0;
}
.formula-properties {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(0, 150px);
  gap: 16px;
}
.formula-reference {
  flex-direction: row;
  flex-wrap: wrap;
  align-items: baseline;
  justify-content: space-between;
  gap: 12px;
  color: var(--prism-muted);
  font-size: 12px;
}
.formula-reference details {
  min-width: 0;
  flex: 1 1 140px;
}
.formula-reference summary {
  cursor: pointer;
}
.formula-details .formula-copy,
.formula-details .formula-remove {
  align-self: flex-start;
  border-color: transparent;
  background: transparent;
  padding: 0 6px;
  font-size: 12px;
}
code {
  display: block;
  margin-top: 12px;
  font-size: 11px;
  overflow-wrap: anywhere;
}
.formula-empty {
  margin: 0;
}
@container (max-width: 660px) {
  .formulas-workspace {
    grid-template-columns: 1fr;
    grid-template-rows: auto minmax(0, 1fr);
    gap: 16px;
  }
  .formula-list {
    max-height: 132px;
  }
}
@container (max-width: 420px) {
  .formula-properties {
    grid-template-columns: 1fr;
  }
}
</style>
