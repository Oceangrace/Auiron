<script setup lang="ts">
import { interfaceExportPrefix } from '../model/interface-format';
import UiMessage from '../../../renderer/components/ui/ui-message.vue';
import { computed, ref, watch, onBeforeUnmount, useId } from 'vue';
import { useI18n } from 'vue-i18n';
import { ImportMode } from './import-mode';
import type { InterfacesModule } from '../interfaces-module';
import type { ImportPreview } from '../model/import-preview';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
const props = defineProps<{ interfaces: InterfacesModule; embedded?: boolean }>();

const emit = defineEmits<{ done: [] }>();

const { t } = useI18n();

const inputId = useId();

const text = ref('');

const parsed = ref<ImportPreview | null>(null);

const error = ref<string | null>(null);

const busy = ref(false);

const copy = ref(ImportMode.Copy);

const fileInput = ref<HTMLInputElement | null>(null);

const state = props.interfaces.session.getState();

const installed = computed(() =>
  state.value.workspace.documents.some((entry) => entry.id === parsed.value?.document.id),
);

const modes = computed(() => [
  { value: ImportMode.Copy, label: t('modules.interfaces.import.copy') },
  { value: ImportMode.Update, label: t('modules.interfaces.import.update') },
]);

const count = computed(
  () => props.interfaces.session.tree.flatten(parsed.value?.document.roots ?? []).length,
);

const canApply = computed(() => parsed.value !== null && !busy.value && !state.value.readOnly);

let inspection = 0;

let inspectionTimer: ReturnType<typeof setTimeout> | null = null;

onBeforeUnmount(() => {
  inspection++;

  if (inspectionTimer !== null) {
    clearTimeout(inspectionTimer);
  }
});

watch(text, () => {
  inspection++;
  busy.value = false;
  parsed.value = null;
  error.value = null;

  if (inspectionTimer !== null) {
    clearTimeout(inspectionTimer);
  }

  if (text.value.trim() !== '') {
    inspectionTimer = setTimeout(() => {
      void inspect();
    }, 180);
  }
});

const inspect = async (): Promise<void> => {
  const revision = ++inspection;

  parsed.value = null;
  error.value = null;
  busy.value = true;

  try {
    const result = await props.interfaces.inspectImport(text.value);

    if (revision === inspection) {
      parsed.value = result;
      copy.value = ImportMode.Copy;
    }
  } catch (reason) {
    if (revision === inspection) {
      error.value = String(reason);
    }
  } finally {
    if (revision === inspection) {
      busy.value = false;
    }
  }
};

const paste = async (): Promise<void> => {
  try {
    text.value = await navigator.clipboard.readText();
  } catch {
    error.value = t('modules.interfaces.import.clipboard');
  }
};

const readFile = async (): Promise<void> => {
  const file = fileInput.value?.files?.[0];

  if (file === undefined) {
    return;
  }

  try {
    if (file.size > 1_000_000 === true) {
      throw new Error(t('modules.interfaces.import.tooLarge'));
    }

    text.value = await file.text();
  } catch (reason) {
    error.value = String(reason);
  } finally {
    if (fileInput.value !== null) {
      fileInput.value.value = '';
    }
  }
};

const readProfile = async (): Promise<void> => {
  if (inspectionTimer !== null) {
    clearTimeout(inspectionTimer);
  }

  const revision = ++inspection;

  busy.value = true;
  error.value = null;

  try {
    const result = await props.interfaces.inspectProfile();

    if (revision === inspection && result !== null) {
      parsed.value = result;
      copy.value = ImportMode.Copy;
    }
  } catch (reason) {
    if (revision === inspection) {
      parsed.value = null;
      error.value = String(reason);
    }
  } finally {
    if (revision === inspection) {
      busy.value = false;
    }
  }
};

const apply = async (): Promise<void> => {
  if (canApply.value === false || parsed.value === null) {
    return;
  }

  busy.value = true;
  error.value = null;

  try {
    const document = parsed.value.document;

    await props.interfaces.importDocument(
      document,
      installed.value && copy.value === ImportMode.Copy,
      t('modules.interfaces.copyName', { name: document.name.slice(0, 100) }),
    );
    emit('done');
  } catch (reason) {
    error.value = String(reason);
  } finally {
    busy.value = false;
  }
};
</script>
<template>
  <section class="import-page">
    <div class="import-content">
      <header v-if="!embedded" class="import-heading">
        <h2>{{ t('modules.interfaces.import.pageTitle') }}</h2>
        <p>{{ t('modules.interfaces.import.hint') }}</p>
      </header>
      <section class="editor-card import-source">
        <div class="section-intro">
          <h3>{{ t('modules.interfaces.redesign.source') }}</h3>
          <p class="section-description">{{ t('modules.interfaces.import.hint') }}</p>
        </div>
        <label :for="inputId">{{ t('modules.interfaces.import.string') }}</label>
        <textarea
          :id="inputId"
          v-model="text"
          class="ui-input import-textarea"
          rows="7"
          :disabled="busy"
          :aria-busy="busy"
          :placeholder="interfaceExportPrefix + '…'"
          :maxlength="1000000"
          spellcheck="false"
          wrap="soft"
        />
        <div class="choice-row import-source-actions">
          <UiButton :disabled="busy" @click="void paste()">
            <svg viewBox="0 0 20 20" aria-hidden="true">
              <path d="M7 4H4v14h12V4h-3 M7 2h6v4H7z M7 10h6 M7 14h6" />
            </svg>
            {{ t('modules.interfaces.import.paste') }}
          </UiButton>
          <UiButton :disabled="busy" @click="fileInput?.click()">
            <svg viewBox="0 0 20 20" aria-hidden="true">
              <path d="M2 5h6l2 2h8v10H2z M2 5V3h6l2 2" />
            </svg>
            {{ t('modules.interfaces.import.file') }}
          </UiButton>
          <UiButton :disabled="busy || text.trim() === ''" @click="void inspect()">
            {{ t('modules.interfaces.import.inspect') }}
          </UiButton>
          <UiButton :disabled="busy" @click="void readProfile()">
            {{ t('modules.interfaces.import.folder') }}
          </UiButton>
          <input
            ref="fileInput"
            hidden
            type="file"
            accept=".txt,.json,text/plain,application/json"
            @change="void readFile()"
          />
        </div>
        <p v-if="error !== null" class="error-note" role="alert"><UiMessage :text="error" /></p>
      </section>
      <section v-if="parsed !== null" class="import-preview editor-card">
        <h3>{{ t('modules.interfaces.import.preview') }}</h3>
        <h4>{{ parsed.document.name }}</h4>
        <div class="interface-summary">
          <span>{{ t('modules.interfaces.information.elementCount', { n: count }) }}</span>
          <span>{{
            t('modules.interfaces.information.formulaCount', { n: parsed.document.formulas.length })
          }}</span>
          <span>{{
            parsed.document.version.trim() === ''
              ? t('modules.interfaces.information.versionMissing')
              : t('modules.interfaces.export.version', { version: parsed.document.version })
          }}</span>
        </div>
        <p class="import-author">
          {{
            parsed.document.author.trim() === ''
              ? t('modules.interfaces.information.authorMissing')
              : t('modules.interfaces.export.author', { author: parsed.document.author })
          }}
        </p>
        <p v-if="parsed.document.description.trim() !== ''" class="import-description">
          {{ parsed.document.description }}
        </p>
        <template v-if="installed">
          <p class="hint">{{ t('modules.interfaces.import.installed') }}</p>
          <UiField class="import-mode" :label="t('modules.interfaces.import.mode')">
            <UiSelect
              v-model="copy"
              :options="modes"
              :label="t('modules.interfaces.import.mode')"
            />
          </UiField>
        </template>
      </section>
      <div class="choice-row import-actions">
        <UiButton
          v-if="parsed !== null"
          class="is-primary"
          :disabled="!canApply"
          @click="void apply()"
          >{{
            t(
              'modules.interfaces.import.' +
                (installed && copy === ImportMode.Update ? 'update' : 'add'),
            )
          }}</UiButton
        >
        <UiButton @click="emit('done')">{{ t('modules.interfaces.cancel') }}</UiButton>
      </div>
    </div>
  </section>
</template>
<style scoped>
.import-page {
  flex: 1;
  min-width: 0;
  overflow: auto;
  padding: 22px 28px 32px;
  scrollbar-gutter: stable;
}
.import-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
  width: 100%;
  max-width: 1100px;
  min-width: 0;
}
.import-page h2,
.import-page h3,
.import-page h4,
.import-page p {
  margin: 0;
}
.import-heading h2 {
  margin-bottom: 8px;
  font-size: 20px;
  font-weight: 600;
}
.import-heading p,
.import-author {
  color: var(--prism-muted);
  font-size: 12px;
}
.import-source > label {
  font-size: 13px;
  font-weight: 500;
}
.import-page .import-textarea {
  min-height: 160px;
  max-height: 320px;
  padding: 12px 20px 12px 12px;
  border: 1px solid var(--prism-card-border);
  background: var(--prism-background);
  resize: vertical;
  overflow-wrap: anywhere;
  font-family: monospace;
  font-size: 12px;
  line-height: 1.8;
  scrollbar-gutter: stable;
}
.import-source-actions,
.import-actions {
  gap: 10px;
}
.import-preview {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.import-preview h3 {
  color: var(--prism-text);
  font-size: 13px;
  font-weight: 600;
}
.import-preview h4 {
  overflow-wrap: anywhere;
  font-size: 18px;
  font-weight: 600;
}
.import-description {
  white-space: pre-wrap;
  overflow-wrap: anywhere;
  color: var(--prism-dim);
  line-height: 1.6;
}
.import-mode {
  width: 100%;
  max-width: none;
  display: grid;
  grid-template-columns: minmax(120px, 1fr) minmax(160px, 300px);
  align-items: center;
  gap: 12px 24px;
}
.import-actions {
  padding-top: 0;
  margin-left: var(--editor-section-label, 210px);
}
.import-page {
  container-type: inline-size;
}
.import-preview .interface-summary {
  color: var(--prism-muted);
}
@container (max-width: 540px) {
  .import-content {
    gap: 16px;
  }
  .import-actions {
    margin-left: 0;
  }
  .import-mode {
    grid-template-columns: 1fr;
    gap: 6px;
  }
}
</style>
