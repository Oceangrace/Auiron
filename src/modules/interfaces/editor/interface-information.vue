<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { InterfaceDocument } from '../model/interface-document';
import type { EditorSession } from './editor-session';
import { HudLocalization } from '../scene/hud-localization';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import type { InterfaceDisplay } from '../model/interface-display';
const props = defineProps<{ document: InterfaceDocument; session: EditorSession }>();

const { t, locale } = useI18n();

const localization = new HudLocalization();

const information = computed(() => localization.document(props.document, locale.value));

const state = props.session.getState();

const count = computed(() => props.session.tree.flatten(props.document.roots).length);

const display = computed<InterfaceDisplay>(
  () =>
    props.document.display ?? { baseWidth: null, baseHeight: null, autoScale: true, scale: 100 },
);

const setDisplay = (change: Partial<InterfaceDisplay>): void => {
  props.session.setDisplay({ ...display.value, ...change });
};

const useScreen = (): void => {
  const screen = props.session.screen();

  setDisplay({ baseWidth: Math.round(screen.width), baseHeight: Math.round(screen.height) });
};
</script>
<template>
  <section class="interface-information editor-page">
    <section class="editor-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.information.main') }}</h3>
        <p class="section-description">{{ t('modules.interfaces.redesign.hints.main') }}</p>
      </div>
      <UiField :label="t('modules.interfaces.name')">
        <UiInput
          :model-value="information.name"
          :label="t('modules.interfaces.name')"
          :disabled="state.readOnly"
          @update:model-value="session.updateInformation({ name: $event }, locale)"
        />
      </UiField>
      <UiField :label="t('modules.interfaces.information.description')">
        <UiInput
          :model-value="information.description"
          :label="t('modules.interfaces.information.description')"
          :placeholder="t('modules.interfaces.information.descriptionHint')"
          :multiline="true"
          :disabled="state.readOnly"
          @update:model-value="session.updateInformation({ description: $event }, locale)"
        />
      </UiField>
    </section>
    <section class="editor-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.information.authorVersion') }}</h3>
        <p class="section-description">{{ t('modules.interfaces.redesign.hints.metadata') }}</p>
      </div>
      <div class="information-metadata field-grid">
        <UiField :label="t('modules.interfaces.information.author')">
          <UiInput
            :model-value="document.author"
            :label="t('modules.interfaces.information.author')"
            :placeholder="t('modules.interfaces.information.authorHint')"
            :disabled="state.readOnly"
            :readonly="document.imported === true"
            @update:model-value="session.updateInformation({ author: $event })"
          />
        </UiField>
        <UiField :label="t('modules.interfaces.information.version')">
          <UiInput
            :model-value="document.version"
            :label="t('modules.interfaces.information.version')"
            placeholder="1.0.0"
            maxlength="64"
            :disabled="state.readOnly"
            :readonly="document.imported === true"
            @update:model-value="session.updateInformation({ version: $event })"
          />
          <small>{{
            t(
              document.imported === true
                ? 'modules.interfaces.information.importedMetadata'
                : 'modules.interfaces.information.versionHint',
            )
          }}</small>
        </UiField>
      </div>
    </section>
    <section class="editor-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.information.display') }}</h3>
      </div>
      <div class="information-metadata field-grid">
        <UiField :label="t('modules.interfaces.information.baseResolution')">
          <div class="information-resolution">
            <template v-if="display.baseWidth !== null && display.baseHeight !== null">
              <UiNumber
                :model-value="display.baseWidth"
                :label="t('modules.interfaces.information.baseWidth')"
                :min="320"
                :max="16384"
                @update:model-value="setDisplay({ baseWidth: $event })"
              />
              <span aria-hidden="true">×</span>
              <UiNumber
                :model-value="display.baseHeight"
                :label="t('modules.interfaces.information.baseHeight')"
                :min="240"
                :max="16384"
                @update:model-value="setDisplay({ baseHeight: $event })"
              />
            </template>
            <span v-else>{{ t('modules.interfaces.information.baseMissing') }}</span>
            <UiButton :disabled="state.readOnly" @click="useScreen">{{
              t('modules.interfaces.information.baseCurrent')
            }}</UiButton>
            <UiButton
              v-if="display.baseWidth !== null"
              :disabled="state.readOnly"
              @click="setDisplay({ baseWidth: null, baseHeight: null })"
              >{{ t('modules.interfaces.information.baseReset') }}</UiButton
            >
          </div>
          <small>{{ t('modules.interfaces.information.baseHint') }}</small>
        </UiField>
        <UiField :label="t('modules.interfaces.information.autoScale')">
          <UiSwitch
            :model-value="display.autoScale"
            :label="t('modules.interfaces.information.autoScaleHint')"
            @update:model-value="setDisplay({ autoScale: $event })"
          />
        </UiField>
        <UiField :label="t('modules.interfaces.information.scale')">
          <UiNumber
            :model-value="display.scale"
            :label="t('modules.interfaces.information.scale')"
            :min="25"
            :max="400"
            :step="5"
            @update:model-value="setDisplay({ scale: $event })"
          />
        </UiField>
      </div>
    </section>
    <section class="editor-card">
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
      </div>
    </section>
  </section>
</template>
<style scoped>
.interface-information :deep(textarea) {
  min-height: 100px;
}
.information-metadata {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.information-metadata > .ui-field {
  display: grid;
  grid-template-columns: minmax(120px, 1fr) minmax(140px, 300px);
  align-items: center;
  gap: 6px 24px;
}
.information-metadata small {
  grid-column: 2;
}
.information-resolution {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 6px;
}
.information-resolution .ui-input {
  width: 90px;
}
.interface-summary {
  padding-top: 16px;
  border-top: 1px solid var(--prism-card-border);
  color: var(--prism-muted);
}
@container (max-width: 540px) {
  .information-metadata > .ui-field {
    grid-template-columns: 1fr;
  }
  .information-metadata small {
    grid-column: 1;
  }
}
</style>
