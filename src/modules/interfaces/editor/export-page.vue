<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { InterfaceDocument } from '../model/interface-document';
import type { InterfacesModule } from '../interfaces-module';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiMessage from '../../../renderer/components/ui/ui-message.vue';
const props = defineProps<{ document: InterfaceDocument; interfaces: InterfacesModule }>();

const { t } = useI18n();

const text = ref('');

const error = ref<string | null>(null);

const generating = ref(false);

const saving = ref(false);

const copied = ref(false);

const savedPath = ref<string | null>(null);

const textarea = ref<HTMLTextAreaElement | null>(null);

const count = computed(() => props.interfaces.session.tree.flatten(props.document.roots).length);

const busy = computed(() => generating.value || saving.value);

watch(
  () => props.document,
  async (document, _previous, onCleanup) => {
    const request = { cancelled: false };

    onCleanup(() => {
      request.cancelled = true;
    });
    generating.value = true;
    text.value = '';
    error.value = null;
    savedPath.value = null;
    copied.value = false;

    try {
      const exported = await props.interfaces.export(document);

      if (request.cancelled === false) {
        text.value = exported;
      }
    } catch (reason) {
      if (request.cancelled === false) {
        error.value = String(reason);
      }
    } finally {
      if (request.cancelled === false) {
        generating.value = false;
      }
    }
  },
  { immediate: true },
);

const copyExport = async (): Promise<void> => {
  const exported = text.value;

  try {
    await navigator.clipboard.writeText(exported);

    if (text.value === exported) {
      copied.value = true;
      error.value = null;
    }
  } catch {
    if (text.value !== exported) {
      return;
    }

    textarea.value?.focus();
    textarea.value?.select();
    error.value = t('modules.interfaces.import.copyManually');
  }
};

const download = async (): Promise<void> => {
  const exported = text.value;

  saving.value = true;
  error.value = null;

  try {
    const path = await props.interfaces.saveExport(exported);

    if (text.value === exported) {
      savedPath.value = path;
    }
  } catch (reason) {
    if (text.value === exported) {
      error.value = String(reason);
    }
  } finally {
    saving.value = false;
  }
};

const downloadProfile = async (): Promise<void> => {
  const document = props.document;

  saving.value = true;
  error.value = null;

  try {
    const path = await props.interfaces.saveProfile(document);

    if (props.document === document) {
      savedPath.value = path;
    }
  } catch (reason) {
    if (props.document === document) {
      error.value = String(reason);
    }
  } finally {
    saving.value = false;
  }
};
</script>
<template>
  <section class="export-page editor-page">
    <section class="editor-card export-overview">
      <header class="export-heading">
        <h3>{{ t('modules.interfaces.export.title') }}</h3>
      </header>
      <div class="interface-summary">
        <span
          ><svg viewBox="0 0 20 20" aria-hidden="true">
            <path d="M2 2h6v6H2z M12 2h6v6h-6z M2 12h6v6H2z M12 12h6v6h-6z" /></svg
          >{{ t('modules.interfaces.information.elementCount', { n: count }) }}</span
        >
        <span
          ><i aria-hidden="true">ƒx</i
          >{{
            t('modules.interfaces.information.formulaCount', { n: document.formulas.length })
          }}</span
        >
        <span
          ><svg viewBox="0 0 20 20" aria-hidden="true">
            <path d="M2 3h7l9 9-6 6-10-10z" />
            <circle cx="6" cy="6" r="1" /></svg
          >{{
            document.version.trim() === ''
              ? t('modules.interfaces.information.versionMissing')
              : t('modules.interfaces.export.version', { version: document.version })
          }}</span
        >
      </div>
      <p class="export-author">
        {{
          document.author.trim() === ''
            ? t('modules.interfaces.information.authorMissing')
            : t('modules.interfaces.export.author', { author: document.author })
        }}
      </p>
    </section>
    <section class="editor-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.export.folderTitle') }}</h3>
      </div>
      <UiButton :disabled="saving" @click="void downloadProfile()">
        {{ t('modules.interfaces.export.folder') }}
      </UiButton>
    </section>
    <section class="editor-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.redesign.exportString') }}</h3>
      </div>
      <div class="export-string-label">
        <label for="interface-export-string">{{ t('modules.interfaces.import.string') }}</label>
        <span>{{ t('modules.interfaces.export.readOnly') }}</span>
      </div>
      <textarea
        id="interface-export-string"
        ref="textarea"
        :value="text"
        class="ui-input export-string"
        readonly
        :aria-busy="generating"
        :aria-label="t('modules.interfaces.import.string')"
        :placeholder="generating ? t('modules.interfaces.loading') : ''"
        spellcheck="false"
        wrap="soft"
      />
      <div class="choice-row export-actions">
        <UiButton class="is-primary" :disabled="busy || text === ''" @click="void copyExport()"
          ><svg viewBox="0 0 20 20" aria-hidden="true"><path d="M7 7h10v10H7z M3 13V3h10" /></svg
          ><span aria-live="polite">{{
            t(
              copied
                ? 'modules.interfaces.formulas.copied'
                : 'modules.interfaces.import.copyString',
            )
          }}</span></UiButton
        >
        <UiButton :disabled="busy || text === ''" @click="void download()"
          ><svg viewBox="0 0 20 20" aria-hidden="true">
            <path d="M3 12v5h14v-5 M10 2v11 M6 9l4 4 4-4" /></svg
          >{{ t('modules.interfaces.import.download') }}</UiButton
        >
      </div>
      <p v-if="savedPath !== null" class="hint export-path" role="status">{{ savedPath }}</p>
      <p v-if="error !== null" class="error-note" role="alert"><UiMessage :text="error" /></p>
    </section>
  </section>
</template>
<style scoped>
.export-page > .editor-card {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 0;
  padding: 0;
}
.export-page > .editor-card + .editor-card {
  padding-top: 24px;
  border-top: 1px solid var(--prism-panel-border);
}
.export-page > .export-overview {
  gap: 12px;
}
.export-page .section-intro {
  position: static;
  width: auto;
}
.export-page .editor-card > .ui-button {
  align-self: flex-start;
}
.export-heading h3 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  line-height: 1.5;
}
.export-page p {
  margin: 0;
}
.export-author,
.export-string-label > span {
  color: var(--prism-muted);
  font-size: 12px;
}
.export-page .interface-summary {
  padding: 0;
  border: 0;
}
.export-string-label {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  gap: 8px;
  font-size: 13px;
  font-weight: 500;
}
.export-page .export-string {
  min-height: 180px;
  max-height: 320px;
  padding: 12px 20px 12px 12px;
  border: 1px solid var(--prism-card-border);
  background: var(--prism-background);
  font-family: monospace;
  font-size: 12px;
  line-height: 1.8;
  overflow-wrap: anywhere;
  scrollbar-gutter: stable;
}
.export-actions {
  gap: 10px;
}
.export-path {
  overflow-wrap: anywhere;
}
</style>
