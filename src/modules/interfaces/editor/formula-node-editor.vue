<script setup lang="ts">
import FieldPicker from './field-picker.vue';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { FormulaNode } from '../model/rules/formula-node';
import type { FormulaDefinition } from '../model/rules/formula-definition';
import { FormulaNodeKind } from '../model/rules/formula-node-kind';
import { FormulaOperation } from '../model/rules/formula-operation';
import { DataKind } from '../model/rules/data-kind';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
const props = withDefaults(
  defineProps<{ node: FormulaNode; formulas: readonly FormulaDefinition[]; depth?: number }>(),
  { depth: 0 },
);

const emit = defineEmits<{ change: [node: FormulaNode] }>();

const { t } = useI18n();

const operationSymbols: Readonly<Record<FormulaOperation, string>> = {
  add: '+',
  subtract: '−',
  multiply: '×',
  divide: '÷',
  minimum: 'min',
  maximum: 'max',
  round: 'round',
  absolute: '|x|',
  clamp: 'clamp',
  equal: '=',
  'not-equal': '≠',
  less: '<',
  'less-or-equal': '≤',
  greater: '>',
  'greater-or-equal': '≥',
  all: '∧',
  any: '∨',
  not: '¬',
  coalesce: '??',
};

const operations = computed(() =>
  Object.values(FormulaOperation).map((value) => ({
    value,
    label: operationSymbols[value] + ' ' + t('modules.interfaces.formulas.operations.' + value),
  })),
);

const kinds = computed(() =>
  Object.values(FormulaNodeKind)
    .filter((value) => props.depth < 5 || value !== FormulaNodeKind.Operation)
    .map((value) => ({ value, label: t('modules.interfaces.formulas.kinds.' + value) })),
);

const choose = (kind: FormulaNodeKind): void => {
  if (kind === props.node.kind) {
    return;
  }

  const values: Record<FormulaNodeKind, FormulaNode> = {
    constant: { kind: FormulaNodeKind.Constant, value: 0 },
    field: { kind: FormulaNodeKind.Field, path: 'player.hp.value' },
    formula: { kind: FormulaNodeKind.Formula, formulaId: props.formulas[0]?.id ?? '' },
    operation: {
      kind: FormulaNodeKind.Operation,
      operation: FormulaOperation.Add,
      operands: [
        { kind: FormulaNodeKind.Constant, value: 0 },
        { kind: FormulaNodeKind.Constant, value: 0 },
      ],
    },
  };

  emit('change', values[kind]);
};

const operation = (value: FormulaOperation): void => {
  let size = 2;

  if (value === FormulaOperation.Clamp) {
    size = 3;
  }

  if (
    [FormulaOperation.Not, FormulaOperation.Round, FormulaOperation.Absolute].includes(value) ===
    true
  ) {
    size = 1;
  }

  emit('change', {
    kind: FormulaNodeKind.Operation,
    operation: value,
    operands: Array.from({ length: size }, (_, index) =>
      props.node.kind === FormulaNodeKind.Operation
        ? (props.node.operands[index] ?? { kind: FormulaNodeKind.Constant, value: 0 })
        : { kind: FormulaNodeKind.Constant, value: 0 },
    ),
  });
};
</script>
<template>
  <div class="formula-node" :class="{ 'formula-node-root': depth === 0 }">
    <nav
      v-if="depth === 0"
      class="formula-source ui-tabs"
      :aria-label="t('modules.interfaces.formulas.expression')"
    >
      <button
        v-for="option in kinds"
        :key="option.value"
        type="button"
        :aria-pressed="node.kind === option.value"
        @click="choose(option.value)"
      >
        {{
          option.value === FormulaNodeKind.Formula
            ? t('modules.interfaces.formulas.reference')
            : option.label
        }}
      </button>
    </nav>
    <UiSelect
      v-else
      :model-value="node.kind"
      :options="kinds"
      :label="t('modules.interfaces.formulas.node')"
      @update:model-value="choose($event)"
    />
    <template v-if="node.kind === FormulaNodeKind.Constant">
      <div class="formula-value-fields">
        <UiField :label="t('modules.interfaces.formulas.valueType')">
          <UiSelect
            :model-value="typeof node.value === 'boolean' ? DataKind.Boolean : DataKind.Number"
            :options="
              [DataKind.Number, DataKind.Boolean].map((value) => ({
                value,
                label: t('modules.interfaces.formulas.types.' + value),
              }))
            "
            :label="t('modules.interfaces.formulas.valueType')"
            @update:model-value="
              emit('change', { ...node, value: $event === DataKind.Boolean ? false : 0 })
            "
          />
        </UiField>
        <UiField :label="t('modules.interfaces.conditions.value')">
          <UiSwitch
            v-if="typeof node.value === 'boolean'"
            :model-value="node.value"
            :label="t('modules.interfaces.conditions.value')"
            @update:model-value="emit('change', { ...node, value: $event })"
          />
          <UiNumber
            v-else
            :model-value="node.value"
            :label="t('modules.interfaces.conditions.value')"
            :min="-1000000000"
            :max="1000000000"
            @update:model-value="emit('change', { ...node, value: $event })"
          />
        </UiField>
      </div>
      <p v-if="depth === 0" class="hint formula-constant-hint">
        {{ t('modules.interfaces.formulas.constantHint') }}
      </p>
    </template>
    <template v-else-if="node.kind === FormulaNodeKind.Field">
      <FieldPicker
        :model-value="node.path"
        :formulas="formulas"
        @choose="emit('change', { ...node, path: $event })"
      />
    </template>
    <UiSelect
      v-else-if="node.kind === FormulaNodeKind.Formula"
      :model-value="node.formulaId"
      :options="formulas.map((value) => ({ value: value.id, label: value.name }))"
      :label="t('modules.interfaces.formulas.reference')"
      @update:model-value="emit('change', { ...node, formulaId: $event })"
    />
    <template v-else>
      <UiSelect
        :model-value="node.operation"
        :options="operations"
        :label="t('modules.interfaces.formulas.operation')"
        @update:model-value="operation($event)"
      >
        <template #option="{ option }">
          <span class="formula-operation-choice">
            <span class="formula-operation-symbol">{{ operationSymbols[option.value] }}</span>
            <span class="formula-operation-label">{{
              t('modules.interfaces.formulas.operations.' + option.value)
            }}</span>
          </span>
        </template>
      </UiSelect>
      <div v-for="(operand, index) in node.operands" :key="index" class="formula-operand">
        <FormulaNodeEditor
          :node="operand"
          :formulas="formulas"
          :depth="depth + 1"
          @change="
            emit('change', {
              ...node,
              operands: node.operands.map((value, at) => (at === index ? $event : value)),
            })
          "
        />
        <UiButton
          v-if="node.operands.length > 2"
          :aria-label="t('modules.interfaces.formulas.removeOperand')"
          @click="
            emit('change', { ...node, operands: node.operands.filter((_, at) => at !== index) })
          "
          >−</UiButton
        >
      </div>
      <UiButton
        v-if="
          node.operands.length < 8 &&
          [
            FormulaOperation.Add,
            FormulaOperation.Multiply,
            FormulaOperation.Minimum,
            FormulaOperation.Maximum,
            FormulaOperation.All,
            FormulaOperation.Any,
            FormulaOperation.Coalesce,
          ].includes(node.operation)
        "
        :aria-label="t('modules.interfaces.formulas.addOperand')"
        @click="
          emit('change', {
            ...node,
            operands: [...node.operands, { kind: FormulaNodeKind.Constant, value: 0 }],
          })
        "
        >＋</UiButton
      >
    </template>
  </div>
</template>
<style scoped>
.formula-node {
  display: grid;
  gap: 12px;
  border-left: 1px solid var(--prism-card-border);
  padding: 8px 0 8px 12px;
}
.formula-node > div {
  min-width: 0;
}
.formula-node-root {
  padding: 0;
  border: 0;
  gap: 16px;
}
.formula-source {
  gap: 4px;
  padding: 4px;
  border: 0;
  border-radius: var(--prism-radius-popup);
  background: var(--prism-background);
}
.formula-source button {
  flex: 1;
  min-width: 0;
  padding: 0 6px;
  border: 1px solid transparent;
  border-radius: var(--prism-radius-control);
  font-size: 12px;
}
.formula-operation-choice {
  display: inline-flex;
  align-items: center;
  gap: 12px;
}
.formula-operation-symbol {
  width: 48px;
  flex-shrink: 0;
  color: var(--prism-text);
  font-size: 16px;
  font-weight: 600;
  text-align: center;
}
.formula-operation-label {
  color: var(--prism-muted);
  font-size: 12px;
}
.formula-value-fields {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}
.formula-constant-hint {
  margin: -8px 0 0;
}
.formula-source button[aria-pressed='true'] {
  border-color: var(--prism-border);
  background: var(--prism-selected);
}
.formula-operand {
  display: grid;
  gap: 8px;
}
.formula-node > .ui-button,
.formula-operand > .ui-button {
  justify-self: start;
  min-width: 30px;
}
@container (max-width: 420px) {
  .formula-value-fields {
    grid-template-columns: 1fr;
  }
}
</style>
