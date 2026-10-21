<script setup lang="ts">
import { computed, inject, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { InterfaceDocument } from '../model/interface-document';
import type { InterfaceLocalization } from '../model/interface-localization';
import type { EditorSession } from './editor-session';
import { HudLocalization } from '../scene/hud-localization';
import { HudLanguageOptions } from './hud-language-options';
import { localizationContextKey } from '../../../renderer/localization/localization-context-key';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';

const props = defineProps<{ document: InterfaceDocument; session: EditorSession }>();

const { t, locale } = useI18n();

const resources = inject(localizationContextKey);

const resolver = new HudLocalization();

const languages = new HudLanguageOptions();

const state = props.session.getState();

const localization = computed<InterfaceLocalization>(
  () => props.document.localization ?? { defaultLocale: '', translations: {} },
);

const choices = computed(() =>
  languages.choices(locale.value, [
    ...(resources?.languages ?? []),
    ...[localization.value.defaultLocale, ...Object.keys(localization.value.translations)]
      .filter((value) => value !== '')
      .map((value) => ({ value, label: value })),
  ]),
);

const codes = computed(() => Object.keys(localization.value.translations));

const target = ref('');

const adding = ref('');

const query = ref('');

const label = (value: string): string =>
  choices.value.find((entry) => entry.value === value)?.label ?? value;

const sourceOptions = computed(() => [
  { value: '', label: t('modules.interfaces.hudTranslations.unspecified') },
  ...choices.value.filter((entry) => !codes.value.includes(entry.value)),
]);

const available = computed(() =>
  choices.value.filter(
    (entry) =>
      entry.value !== localization.value.defaultLocale && !codes.value.includes(entry.value),
  ),
);

watch(
  codes,
  (values) => {
    if (values.includes(target.value) === false) {
      target.value = values[0] ?? '';
    }
  },
  { immediate: true },
);

watch(
  available,
  (values) => {
    if (values.some((entry) => entry.value === adding.value) === false) {
      adding.value = values[0]?.value ?? '';
    }
  },
  { immediate: true },
);

const sources = computed(() => resolver.sources(props.document));

const dictionary = computed(() =>
  Object.hasOwn(localization.value.translations, target.value)
    ? (localization.value.translations[target.value] ?? {})
    : {},
);

const translation = (source: string): string =>
  Object.hasOwn(dictionary.value, source) ? (dictionary.value[source] ?? '') : '';

const rows = computed(() =>
  [...new Set([...sources.value, ...Object.keys(dictionary.value)])].filter((source) =>
    (source + ' ' + translation(source))
      .toLocaleLowerCase(locale.value)
      .includes(query.value.toLocaleLowerCase(locale.value)),
  ),
);

const completed = computed(
  () => sources.value.filter((source) => translation(source) !== '').length,
);

const change = (value: InterfaceLocalization): void => {
  props.session.setLocalization(value);
};

const add = (): void => {
  if (adding.value === '' || codes.value.length >= 32 === true) {
    return;
  }

  const code = adding.value;

  change({
    ...localization.value,
    translations: { ...localization.value.translations, [code]: {} },
  });
  target.value = code;
};

const remove = (): void => {
  const translations = Object.fromEntries(
    Object.entries(localization.value.translations).filter(([code]) => code !== target.value),
  );

  change({ ...localization.value, translations });
};

const update = (source: string, value: string): void => {
  const entries = Object.entries(dictionary.value).filter(([key]) => key !== source);

  if (value !== '') {
    entries.push([source, value]);
  }

  change({
    ...localization.value,
    translations: {
      ...localization.value.translations,
      [target.value]: Object.fromEntries(entries),
    },
  });
};
</script>
<template>
  <section class="editor-page hud-translations">
    <section class="editor-card">
      <UiField :label="t('modules.interfaces.hudTranslations.sourceLanguage')">
        <UiSelect
          :model-value="localization.defaultLocale"
          :options="sourceOptions"
          :label="t('modules.interfaces.hudTranslations.sourceLanguage')"
          :disabled="state.readOnly"
          @update:model-value="change({ ...localization, defaultLocale: $event })"
        />
      </UiField>
      <div class="translation-toolbar">
        <UiField :label="t('modules.interfaces.hudTranslations.addLanguage')">
          <UiSelect
            v-model="adding"
            :options="available"
            :label="t('modules.interfaces.hudTranslations.addLanguage')"
            :disabled="state.readOnly || codes.length >= 32"
          />
        </UiField>
        <UiButton :disabled="state.readOnly || adding === '' || codes.length >= 32" @click="add">{{
          t('modules.interfaces.hudTranslations.add')
        }}</UiButton>
      </div>
    </section>
    <section v-if="codes.length > 0" class="editor-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.hudTranslations.texts') }}</h3>
        <p class="section-description">{{ t('modules.interfaces.hudTranslations.fallback') }}</p>
      </div>
      <div class="translation-toolbar">
        <UiField :label="t('modules.interfaces.hudTranslations.language')">
          <UiSelect
            v-model="target"
            :options="codes.map((value) => ({ value, label: label(value) }))"
            :label="t('modules.interfaces.hudTranslations.language')"
          />
        </UiField>
        <UiButton :disabled="state.readOnly" @click="remove">{{
          t('modules.interfaces.hudTranslations.remove')
        }}</UiButton>
      </div>
      <UiInput
        v-model="query"
        :label="t('modules.interfaces.hudTranslations.search')"
        :placeholder="t('modules.interfaces.hudTranslations.search')"
      />
      <p class="hint">
        {{
          t('modules.interfaces.hudTranslations.progress', {
            done: completed,
            total: sources.length,
          })
        }}
      </p>
      <div v-for="source in rows.slice(0, 100)" :key="source" class="translation-row">
        <div class="translation-original">
          <span class="ui-label">{{ t('modules.interfaces.hudTranslations.original') }}</span>
          <p>{{ source }}</p>
          <small v-if="!sources.includes(source)">{{
            t('modules.interfaces.hudTranslations.unused')
          }}</small>
        </div>
        <UiField :label="label(target)">
          <UiInput
            :model-value="translation(source)"
            :label="label(target) + ' · ' + source"
            :placeholder="t('modules.interfaces.hudTranslations.originalFallback')"
            :multiline="true"
            :disabled="state.readOnly"
            maxlength="2048"
            @update:model-value="update(source, $event)"
          />
          <small
            v-if="!resolver.compatible(source, translation(source))"
            role="alert"
            class="error-note"
            >{{ t('modules.interfaces.hudTranslations.fieldsWarning') }}</small
          >
        </UiField>
      </div>
      <p v-if="rows.length > 100" class="hint">
        {{ t('modules.interfaces.hudTranslations.more') }}
      </p>
      <p v-if="rows.length === 0" class="hint">
        {{ t('modules.interfaces.hudTranslations.empty') }}
      </p>
    </section>
  </section>
</template>
<style scoped>
.hud-translations > .editor-card {
  min-height: 160px;
  padding-left: var(--editor-section-label, 210px);
}
.hud-translations > .editor-card:first-child {
  min-height: 0;
  padding-left: 0;
}
.hud-translations > .editor-card + .editor-card {
  padding-top: 24px;
  border-top: 1px solid var(--prism-panel-border);
}
.hud-translations > .editor-card + .editor-card > .section-intro {
  top: 28px;
}
.translation-toolbar {
  display: flex;
  align-items: end;
  gap: 12px;
  flex-wrap: wrap;
}
.translation-toolbar > .ui-field {
  flex: 1;
  min-width: 180px;
}
.translation-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
  gap: 20px;
  padding: 18px 0;
  border-top: 1px solid var(--prism-panel-border);
}
.translation-row :deep(textarea) {
  min-height: 70px;
}
.translation-original {
  min-width: 0;
  overflow-wrap: anywhere;
}
.translation-original p {
  white-space: pre-wrap;
}
@container (max-width:540px) {
  .hud-translations > .editor-card {
    padding-left: 0;
    min-height: 0;
  }
  .hud-translations > .editor-card > .section-intro {
    position: static;
    width: auto;
  }
  .translation-row {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
