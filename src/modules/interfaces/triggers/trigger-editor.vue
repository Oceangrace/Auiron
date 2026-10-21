<script setup lang="ts">
import { computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from '../editor/editor-session';
import type { CatalogClient } from './catalog-client';
import type { CatalogItem } from '../model/catalog-item';
import type { CatalogForm } from '../model/catalog-form';
import { CatalogScope } from '../model/catalog-scope';
import { TriggerSubject } from '../model/trigger-subject';
import { ObservationKind } from '../model/observation-kind';
import { TriggerController } from './trigger-controller';
import CatalogPicker from './catalog-picker.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import './trigger-editor.css';
const props = defineProps<{ widget: WidgetNode; session: EditorSession; catalog: CatalogClient }>();

const { t } = useI18n();

const controller = new TriggerController();

watch(
  () => props.widget.trigger,
  (trigger) => {
    controller.restore(trigger, props.widget.appearance.iconPath);
  },
  { immediate: true },
);

const draft = controller.getState();

const needsContext = computed(
  () =>
    draft.value.scope === CatalogScope.EnemySkills ||
    draft.value.scope === CatalogScope.EnemyEffects,
);

const subjects = computed(() => [
  { value: TriggerSubject.SelfPlayer, label: t('modules.interfaces.self') },
  { value: TriggerSubject.Enemy, label: t('modules.interfaces.enemy') },
]);

const observations = computed(() => [
  { value: ObservationKind.EffectPresent, label: t('modules.interfaces.effectPresent') },
  { value: ObservationKind.SkillUsed, label: t('modules.interfaces.skillUsed') },
  ...[
    ObservationKind.SkillState,
    ObservationKind.Cooldown,
    ObservationKind.SkillReady,
    ObservationKind.ComboAvailable,
  ].map((value) => ({ value, label: t('modules.interfaces.skillStates.' + value) })),
]);

const apply = (): void => {
  const trigger = controller.definition();

  if (trigger !== null) {
    props.session.updateSelected((widget) => ({
      ...widget,
      trigger,
      appearance: { ...widget.appearance, iconPath: draft.value.iconPath },
    }));
  }
};

const choose = (item: CatalogItem, form: CatalogForm, version: string): void => {
  controller.choose(item, form, version);
  apply();
};

const observe = (value: ObservationKind): void => {
  controller.setObservation(value);
  apply();
};

const seconds = (value: number): void => {
  controller.setSeconds(value);
  apply();
};
</script>
<template>
  <div class="trigger-editor">
    <section class="trigger-step inspector-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.stepWho') }}</h3>
        <p class="section-description">{{ t('modules.interfaces.redesign.hints.who') }}</p>
      </div>
      <UiField :label="t('modules.interfaces.rules.unit')"
        ><UiSelect
          :model-value="draft.subject"
          :options="subjects"
          :label="t('modules.interfaces.rules.unit')"
          @update:model-value="controller.setSubject($event)"
      /></UiField>
      <CatalogPicker
        v-if="needsContext"
        :client="catalog"
        :scope="CatalogScope.Enemies"
        :context-key="null"
        :selected-key="draft.contextKey"
        :label="t('modules.interfaces.findEnemy')"
        :enemies="true"
        @choose="controller.setEnemy($event)"
      />
    </section>
    <section class="trigger-step inspector-card">
      <div class="section-intro">
        <h3>{{ t('modules.interfaces.stepWhat') }}</h3>
        <p class="section-description">{{ t('modules.interfaces.redesign.hints.what') }}</p>
      </div>
      <UiField
        v-if="draft.subject === TriggerSubject.SelfPlayer"
        :label="t('modules.interfaces.showWhen')"
        ><UiSelect
          :model-value="draft.observation"
          :options="observations"
          :label="t('modules.interfaces.showWhen')"
          @update:model-value="observe"
      /></UiField>
      <UiField
        v-if="
          draft.subject === TriggerSubject.SelfPlayer &&
          draft.observation === ObservationKind.EffectPresent
        "
        :label="t('modules.interfaces.effectCatalog')"
        ><UiSelect
          :model-value="draft.scope"
          :options="[
            { value: CatalogScope.Effects, label: t('modules.interfaces.commonEffects') },
            { value: CatalogScope.EnemyEffects, label: t('modules.interfaces.enemyEffects') },
          ]"
          :label="t('modules.interfaces.effectCatalog')"
          @update:model-value="controller.setEffectScope($event)"
      /></UiField>
      <CatalogPicker
        v-if="!needsContext || draft.contextKey !== null"
        :key="draft.scope + ':' + draft.contextKey"
        :client="catalog"
        :scope="draft.scope"
        :context-key="draft.contextKey"
        :selected-key="draft.selection?.entryKey ?? null"
        :label="
          t(
            draft.observation === ObservationKind.EffectPresent
              ? 'modules.interfaces.findEffect'
              : 'modules.interfaces.findSkill',
          )
        "
        @choose="choose"
      />
      <UiField
        v-if="draft.observation === ObservationKind.SkillUsed"
        :label="t('modules.interfaces.duration')"
        ><UiNumber
          :model-value="draft.seconds"
          :label="t('modules.interfaces.duration')"
          :min="0.1"
          :max="3600"
          :step="0.1"
          @update:model-value="seconds"
      /></UiField>
      <UiButton
        v-if="widget.trigger !== null"
        @click="session.updateSelected((node) => ({ ...node, trigger: null }))"
        >{{ t('modules.interfaces.clearTrigger') }}</UiButton
      >
    </section>
  </div>
</template>
