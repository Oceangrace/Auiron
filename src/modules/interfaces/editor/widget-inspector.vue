<script setup lang="ts">
import TemplateField from './template-field.vue';
import LoadFields from './load-fields.vue';
import CompoundTriggerFields from './compound-trigger-fields.vue';
import { computed, ref } from 'vue';
import type { AppearancePreview } from './appearance-preview';
import WidgetStyleFields from './widget-style-fields.vue';
import ConditionsEditor from './conditions-editor.vue';
import { useI18n } from 'vue-i18n';
import type { WidgetNode } from '../model/widget-node';
import type { EditorSession } from './editor-session';
import type { CatalogClient } from '../triggers/catalog-client';
import { WidgetKind } from '../model/widget-kind';
import { InspectorTab } from './inspector-tab';
import TriggerEditor from '../triggers/trigger-editor.vue';
import UiButton from '../../../renderer/components/ui/ui-button.vue';
import UiField from '../../../renderer/components/ui/ui-field.vue';
import UiInput from '../../../renderer/components/ui/ui-input.vue';
import AnchorFields from './anchor-fields.vue';
import { AnchorMode } from '../model/anchor-mode';
import UiNumber from '../../../renderer/components/ui/ui-number.vue';
import UiColor from '../../../renderer/components/ui/ui-color.vue';
import UiSlider from '../../../renderer/components/ui/ui-slider.vue';
import UiSelect from '../../../renderer/components/ui/ui-select.vue';
import UiSwitch from '../../../renderer/components/ui/ui-switch.vue';
import UiIconButton from '../../../renderer/components/ui/ui-icon-button.vue';
import UiPageHeader from '../../../renderer/components/ui/ui-page-header.vue';
import { HudLocalization } from '../scene/hud-localization';

const props = defineProps<{
  widget: WidgetNode;
  session: EditorSession;
  catalog: CatalogClient;
  preview: AppearancePreview;
}>();

const emit = defineEmits<{ openInterface: []; addChild: [] }>();

const state = props.session.getState();

const actionsOpen = ref(false);

const { t, locale } = useI18n();

const localization = new HudLocalization();

const label = (name: string): string =>
  localization.text(name, props.session.document()?.localization, locale.value);

const tab = defineModel<InspectorTab>('tab', { default: InspectorTab.Display });

const groups = computed(() =>
  props.session.tree
    .flatten(props.session.document()?.roots ?? [])
    .filter(
      (node) =>
        props.session.tree.isGroup(node) &&
        node.id !== props.widget.id &&
        props.session.tree.find(props.widget.children, node.id) === null,
    ),
);

const parent = computed(() =>
  props.session.tree.parent(props.session.document()?.roots ?? [], props.widget.id),
);

const parents = computed(() => [
  { value: '', label: t('modules.interfaces.root') },
  ...groups.value.map((node) => ({
    value: node.id,
    label: props.session.tree
      .path(props.session.document()?.roots ?? [], node.id)
      .map((entry) => label(entry.name))
      .join(' › '),
  })),
]);
</script>
<template>
  <section class="widget-inspector">
    <UiPageHeader :title="label(widget.name)">
      <template #actions>
        <UiIconButton
          :label="t('modules.interfaces.enabled')"
          :active="widget.enabled"
          :aria-pressed="widget.enabled"
          :disabled="state.readOnly"
          @click="session.toggleWidget(widget.id)"
        >
          <svg viewBox="0 0 20 20">
            <path d="M2 10s3-5 8-5 8 5 8 5-3 5-8 5-8-5Z" />
            <circle cx="10" cy="10" r="2" />
            <path v-if="!widget.enabled" d="m3 17 14-14" />
          </svg>
        </UiIconButton>
        <div class="widget-actions-menu" @keydown.esc.stop="actionsOpen = false">
          <UiIconButton
            :label="t('modules.interfaces.redesign.actions')"
            :aria-expanded="actionsOpen"
            @click="actionsOpen = !actionsOpen"
          >
            <svg viewBox="0 0 20 20">
              <circle cx="4" cy="10" r="1" />
              <circle cx="10" cy="10" r="1" />
              <circle cx="16" cy="10" r="1" />
            </svg>
          </UiIconButton>
          <template v-if="actionsOpen">
            <button
              type="button"
              class="widget-menu-backdrop"
              :aria-label="t('modules.interfaces.cancel')"
              @click="actionsOpen = false"
            />
            <div class="widget-menu-items">
              <UiButton
                v-if="session.tree.isGroup(widget)"
                :disabled="state.readOnly"
                @click="
                  emit('addChild');
                  actionsOpen = false;
                "
                >{{ t('modules.interfaces.create.new') }}</UiButton
              >
              <UiButton
                :disabled="state.readOnly"
                @click="
                  session.duplicateSelected(
                    t('modules.interfaces.copyName', { name: widget.name }),
                  );
                  actionsOpen = false;
                "
                >{{ t('modules.interfaces.duplicate') }}</UiButton
              >
              <UiButton
                :danger="true"
                :disabled="state.readOnly"
                @click="
                  session.removeSelected();
                  actionsOpen = false;
                "
                >{{ t('modules.interfaces.remove') }}</UiButton
              >
            </div>
          </template>
        </div>
      </template>
    </UiPageHeader>
    <nav class="ui-tabs" :aria-label="t('modules.interfaces.showWhen')">
      <button
        v-for="value in Object.values(InspectorTab)"
        :key="value"
        type="button"
        :aria-pressed="tab === value"
        @click="tab = value"
      >
        {{ t('modules.interfaces.tabs.' + value) }}
      </button>
    </nav>
    <div class="inspector-scroll">
      <template v-if="tab === InspectorTab.Display">
        <section class="inspector-section">
          <section class="inspector-card inspector-content-fields">
            <div class="section-intro">
              <h3>{{ t('modules.interfaces.inspectorSections.content') }}</h3>
              <p class="section-description">
                {{ t('modules.interfaces.redesign.hints.content') }}
              </p>
            </div>
            <UiField :label="t('modules.interfaces.name')"
              ><UiInput
                :model-value="widget.name"
                :label="t('modules.interfaces.name')"
                @update:model-value="session.renameWidget($event)"
            /></UiField>
            <UiField
              v-if="session.tree.isGroup(widget) === false"
              :label="t('modules.interfaces.text')"
              ><TemplateField
                :formulas="session.document()?.formulas ?? []"
                :model-value="widget.appearance.text"
                :label="t('modules.interfaces.text')"
                :multiline="true"
                @update:model-value="
                  session.updateSelected((node) => ({
                    ...node,
                    appearance: { ...node.appearance, text: $event },
                  }))
                "
            /></UiField>
          </section>
          <WidgetStyleFields
            :widget="widget"
            :session="session"
            :catalog="catalog"
            :tab="InspectorTab.Display"
          />
        </section>
      </template>
      <section v-else-if="tab === InspectorTab.Style" class="inspector-section inspector-style">
        <section class="inspector-card colors-section">
          <h3>
            {{
              t(
                session.tree.isGroup(widget)
                  ? 'modules.interfaces.background'
                  : 'modules.interfaces.inspectorSections.colors',
              )
            }}
          </h3>
          <UiSwitch
            v-if="session.tree.isGroup(widget)"
            :model-value="widget.group.fillEnabled"
            :label="t('modules.interfaces.visual.fillEnabled')"
            @update:model-value="
              session.updateSelected((node) => ({
                ...node,
                group: { ...node.group, fillEnabled: $event },
              }))
            "
          />
          <div class="field-grid">
            <UiField
              v-if="session.tree.isGroup(widget) === false"
              :label="t('modules.interfaces.color')"
              ><UiColor
                :model-value="widget.appearance.color"
                :label="t('modules.interfaces.color')"
                @update:model-value="
                  session.updateSelected((node) => ({
                    ...node,
                    appearance: { ...node.appearance, color: $event },
                  }))
                "
            /></UiField>
            <UiField :label="t('modules.interfaces.background')"
              ><UiColor
                :model-value="widget.appearance.background"
                :label="t('modules.interfaces.background')"
                @update:model-value="
                  session.updateSelected((node) => ({
                    ...node,
                    appearance: { ...node.appearance, background: $event },
                    group: { ...node.group, fillOpacity: null },
                    style: { ...node.style, backgroundOpacity: null },
                  }))
                "
            /></UiField>
            <UiField
              v-if="session.tree.isGroup(widget)"
              :label="t('modules.interfaces.advanced.fillOpacity')"
              ><UiSlider
                :model-value="
                  widget.group.fillOpacity ??
                  (widget.appearance.background.length === 9
                    ? Math.round(parseInt(widget.appearance.background.slice(7), 16) / 2.55)
                    : 100)
                "
                :label="t('modules.interfaces.advanced.fillOpacity')"
                @update:model-value="
                  session.updateSelected((node) => ({
                    ...node,
                    group: { ...node.group, fillOpacity: $event },
                    appearance: {
                      ...node.appearance,
                      background:
                        node.appearance.background.slice(0, 7) +
                        Math.round($event * 2.55)
                          .toString(16)
                          .padStart(2, '0'),
                    },
                  }))
                "
            /></UiField>
          </div>
        </section>
        <WidgetStyleFields
          :widget="widget"
          :session="session"
          :catalog="catalog"
          :tab="InspectorTab.Style"
        />
      </section>
      <section v-else-if="tab === InspectorTab.Trigger" class="inspector-section inspector-trigger">
        <TriggerEditor
          v-if="widget.rules === null || widget.trigger !== null"
          :key="widget.id"
          :widget="widget"
          :session="session"
          :catalog="catalog"
        />
        <CompoundTriggerFields
          :widget="widget"
          :session="session"
          :catalog="catalog"
          :preview="preview"
        />
      </section>
      <LoadFields v-else-if="tab === InspectorTab.Load" :widget="widget" :session="session" />
      <ConditionsEditor
        v-else-if="tab === InspectorTab.Conditions"
        :widget="widget"
        :session="session"
        :preview="preview"
      />
      <section v-else class="inspector-section">
        <section class="inspector-card">
          <div class="section-intro">
            <h3>{{ t('modules.interfaces.inspectorSections.position') }}</h3>
          </div>
          <div class="field-grid">
            <UiField
              v-if="parent?.kind !== WidgetKind.DynamicGroup"
              :label="t('modules.interfaces.x')"
              ><UiNumber
                :model-value="session.coordinate('x')"
                :label="t('modules.interfaces.x')"
                :min="-100000"
                :max="100000"
                @update:model-value="session.updateCoordinate('x', $event)"
            /></UiField>
            <UiField
              v-if="parent?.kind !== WidgetKind.DynamicGroup"
              :label="t('modules.interfaces.y')"
              ><UiNumber
                :model-value="session.coordinate('y')"
                :label="t('modules.interfaces.y')"
                :min="-100000"
                :max="100000"
                @update:model-value="session.updateCoordinate('y', $event)"
            /></UiField>
            <UiField
              v-if="
                parent === null ||
                parent.kind === WidgetKind.DynamicGroup ||
                (widget.anchors.horizontal !== AnchorMode.Stretch &&
                  widget.anchors.widthPercent === null)
              "
              :label="t('modules.interfaces.width')"
              ><UiNumber
                :model-value="widget.layout.width"
                :label="t('modules.interfaces.width')"
                :min="8"
                :max="8192"
                @update:model-value="
                  session.updateSelected((node) => ({
                    ...node,
                    layout: { ...node.layout, width: $event },
                  }))
                "
            /></UiField>
            <UiField
              v-if="
                parent === null ||
                parent.kind === WidgetKind.DynamicGroup ||
                (widget.anchors.vertical !== AnchorMode.Stretch &&
                  widget.anchors.heightPercent === null)
              "
              :label="t('modules.interfaces.height')"
              ><UiNumber
                :model-value="widget.layout.height"
                :label="t('modules.interfaces.height')"
                :min="8"
                :max="8192"
                @update:model-value="
                  session.updateSelected((node) => ({
                    ...node,
                    layout: { ...node.layout, height: $event },
                  }))
                "
            /></UiField>
          </div>
        </section>
        <WidgetStyleFields
          :widget="widget"
          :session="session"
          :catalog="catalog"
          :tab="InspectorTab.Placement"
        />
        <AnchorFields
          v-if="parent?.kind === WidgetKind.Group"
          :session="session"
          :widget="widget"
        />
        <section class="inspector-card">
          <div class="section-intro">
            <h3>{{ t('modules.interfaces.inspectorSections.hierarchy') }}</h3>
            <p class="section-description">
              {{ t('modules.interfaces.redesign.hints.hierarchy') }}
            </p>
          </div>
          <UiField :label="t('modules.interfaces.parent')"
            ><UiSelect
              :model-value="parent?.id ?? ''"
              :options="parents"
              :label="t('modules.interfaces.parent')"
              @update:model-value="session.reparent($event === '' ? null : $event)"
          /></UiField>
          <div class="choice-row">
            <UiButton @click="session.reorder(-1)">{{ t('modules.interfaces.front') }}</UiButton
            ><UiButton @click="session.reorder(1)">{{ t('modules.interfaces.back') }}</UiButton>
          </div>
        </section>
      </section>
    </div>
  </section>
</template>
