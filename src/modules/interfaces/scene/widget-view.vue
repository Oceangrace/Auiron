<script setup lang="ts">
import { computed, ref, watch, onBeforeUnmount } from 'vue';
import type { CSSProperties } from 'vue';
import type { EditorSession } from '../editor/editor-session';
import { SoundCue } from '../model/sound-cue';
import WidgetGroupHeader from './widget-group-header.vue';
import { ChildLayout } from './child-layout';
import type { PreviewVisual } from '../model/preview-visual';
import type { PreviewPlayback } from '../model/preview-playback';
import BarWidget from './bar-widget.vue';
import type { WidgetLayout } from '../model/widget-layout';
import type { WidgetNode } from '../model/widget-node';
import { AnimationWhen } from '../model/animation-when';
import { AnimationTrigger } from './animation-trigger';
import { GroupFlow } from '../model/group-flow';
import { WidgetKind } from '../model/widget-kind';
import { WidgetGeometry } from './widget-geometry';
import { WidgetClassColors } from './widget-class-colors';
import { GroupScale } from './group-scale';
import IconWidget from './icon-widget.vue';
import GroupEffectIcons from './group-effect-icons.vue';
import { GroupDataSource } from '../model/rules/group-data-source';
import TextWidget from './text-widget.vue';
import type { WidgetPointer } from '../editor/widget-pointer';
const props = withDefaults(
  defineProps<{
    instance?: PreviewVisual | null;
    session: EditorSession;
    parentLayout?: WidgetLayout | null;
    parent?: WidgetNode | null;
    visuals?: Readonly<Record<string, PreviewVisual>>;
    widget: WidgetNode;
    selectedId: string | null;
    editable: boolean;
    scale: number;
    pointer: WidgetPointer | null;
    preview: boolean;
    passive?: boolean;
    playback?: PreviewPlayback | null;
  }>(),
  {
    parentLayout: null,
    parent: null,
    visuals: () => ({}),
    instance: null,
    passive: false,
    playback: null,
  },
);

const geometry = new WidgetGeometry();

const groupScale = new GroupScale();

const factor = computed(() => groupScale.factor(props.widget));

const children = new ChildLayout();

const effectGroup = computed(() => props.widget.binding?.source === GroupDataSource.Effects);

const runtime = computed(() => props.instance ?? props.visuals[props.widget.id] ?? null);

const childVisuals = computed(() => ({ ...props.visuals, ...runtime.value?.childVisuals }));

const classColors = new WidgetClassColors();

const visual = computed(() => classColors.apply(props.widget, runtime.value));

const ticking = computed(() =>
  runtime.value?.simulating === true ? props.playback?.running === true : props.preview === false,
);

const sampledAt = computed(() =>
  runtime.value?.simulating === true ? (props.playback?.sampledAt ?? null) : null,
);

const collapsed = ref(false);

const selection = props.session.getState();

const selected = computed(
  () => props.editable && selection.value.selectedIds.includes(props.widget.id),
);

const sound = props.session.sound;

const animationTrigger = new AnimationTrigger();

const parentVisible = computed(
  () =>
    props.parent === null ||
    children.visible(props.widget, props.parent, props.editable, props.visuals),
);

const shown = computed(
  () =>
    (props.editable || props.widget.enabled) &&
    ((props.preview && runtime.value?.simulating !== true) || runtime.value?.visible === true) &&
    parentVisible.value,
);

watch(shown, (value, previous) => {
  if (props.passive === false && props.preview === false && value !== previous) {
    void sound.play({
      cue: value ? props.widget.sounds.show : props.widget.sounds.hide,
      volume: props.widget.sounds.volume,
      text: value ? props.widget.sounds.showText : props.widget.sounds.hideText,
      voice: props.widget.sounds.voice,
      rate: props.widget.sounds.voiceRate,
      owner: props.widget.id,
    });
  }
});
watch(
  () => runtime.value?.sound,
  (value, previous) => {
    if (
      props.passive === false &&
      props.preview === false &&
      value !== undefined &&
      value !== previous &&
      value !== SoundCue.None
    ) {
      void sound.play({
        cue: value,
        volume: props.widget.sounds.volume,
        text: runtime.value?.speechText ?? '',
        voice: props.widget.sounds.voice,
        rate: props.widget.sounds.voiceRate,
        owner: props.widget.id,
      });
    }
  },
);
const transient = ref(false);

let animationTimer: ReturnType<typeof setTimeout> | null = null;

const pulse = (): void => {
  transient.value = true;

  if (animationTimer !== null) {
    clearTimeout(animationTimer);
  }

  animationTimer = setTimeout(() => {
    transient.value = false;
  }, props.widget.style.animationDuration);
};

watch(
  shown,
  (value, previous) => {
    if (
      value === true &&
      previous !== true &&
      props.widget.style.animationWhen === AnimationWhen.OnShow
    ) {
      pulse();
    }
  },
  { immediate: true },
);
watch(
  () => runtime.value?.animationValue,
  (value, previous) => {
    if (
      props.widget.style.animationWhen === AnimationWhen.ValueChanged &&
      animationTrigger.changed(value, previous, props.widget.style.animationChange) === true
    ) {
      pulse();
    }
  },
);

onBeforeUnmount(() => {
  if (animationTimer !== null) {
    clearTimeout(animationTimer);
  }
});
const emit = defineEmits<{ edit: [id: string] }>();

const group = computed(
  () => props.widget.kind === WidgetKind.Group || props.widget.kind === WidgetKind.DynamicGroup,
);

const instanceVisuals = computed(() =>
  props.instance === null ? props.visuals : { ...props.visuals, [props.widget.id]: props.instance },
);

const baseLayout = computed(() => {
  if (props.parent !== null && props.parentLayout !== null) {
    return children.resolve(
      props.widget,
      props.parent,
      props.parentLayout,
      props.editable,
      instanceVisuals.value,
    );
  }

  return props.widget.layout;
});

const instanceOffset = computed(() => {
  if (props.parent?.group.flow === GroupFlow.Timeline) {
    return { x: 0, y: 0 };
  }

  return { x: runtime.value?.offsetX ?? 0, y: runtime.value?.offsetY ?? 0 };
});

const layout = computed(() => ({
  ...baseLayout.value,
  x: baseLayout.value.x + instanceOffset.value.x,
  y: baseLayout.value.y + instanceOffset.value.y,
  height: collapsed.value ? props.widget.group.headerHeight : baseLayout.value.height,
}));

const contentStyle = computed(() => ({
  width: String(layout.value.width) + 'px',
  height: String(layout.value.height) + 'px',
  transform: group.value ? `scale(${String(factor.value)})` : undefined,
}));

const decoration = computed(() => ({
  ...geometry.style(visual.value, layout.value),
  ...(effectGroup.value ? { boxShadow: 'none' } : {}),
}));

const movingHistory = computed(
  () => props.instance !== null && props.parent?.binding?.source === GroupDataSource.SkillHistory,
);

const historyGroup = computed(() => props.widget.binding?.source === GroupDataSource.SkillHistory);

const movement = computed<CSSProperties>(() => {
  const transitionDuration = String(props.parent?.group.motionMs ?? 0) + 'ms';

  if (movingHistory.value === false) {
    return { transitionProperty: 'left, top', transitionDuration };
  }

  // Move the icon's composited layer; left/top animation repaints transparent backing pixels.
  return {
    left: '0px',
    top: '0px',
    transform: `translate3d(${String(decoration.value.left)}, ${String(decoration.value.top)}, 0)`,
    transitionProperty: 'transform, opacity',
    transitionDuration,
    transitionTimingFunction: 'linear',
    willChange: 'transform, opacity',
  };
});

const style = computed(() => ({
  ...decoration.value,
  ...movement.value,
  '--motion-time': String(props.widget.style.motionDuration) + 'ms',
  zIndex: selected.value ? 100 + props.widget.style.layer : props.widget.style.layer,
  animationName: (
    props.widget.style.animationWhen === AnimationWhen.OnShow ||
    props.widget.style.animationWhen === AnimationWhen.ValueChanged
      ? transient.value === false
      : runtime.value?.animationActive === false
  )
    ? 'none'
    : decoration.value.animationName,
}));

const movable = computed(
  () =>
    props.instance === null &&
    props.parent?.kind !== WidgetKind.DynamicGroup &&
    props.widget.group.locked === false,
);

const beginMove = (event: PointerEvent): void => {
  if (props.editable === false || (event.ctrlKey === true && selected.value === false)) {
    return;
  }

  props.pointer?.begin(
    event,
    props.widget,
    props.scale,
    false,
    layout.value,
    movable.value,
    factor.value,
  );
};

const captureMove = (event: PointerEvent): void => {
  if (
    selected.value === false ||
    (event.ctrlKey === false && selection.value.selectedIds.length <= 1 === true)
  ) {
    return;
  }

  if (
    event.target instanceof Element === true &&
    event.target.closest('[data-resize-id]') !== null
  ) {
    return;
  }

  beginMove(event);
};
</script>
<template>
  <template v-if="instance === null && !effectGroup && (runtime?.instances.length ?? 0) > 0">
    <WidgetView
      v-for="entry in runtime?.instances ?? []"
      :key="entry.key"
      :widget="widget"
      :session="session"
      :parent="parent"
      :parent-layout="parentLayout"
      :selected-id="selectedId"
      :editable="editable"
      :scale="scale"
      :pointer="pointer"
      :preview="preview"
      :passive="passive"
      :playback="playback"
      :visuals="visuals"
      :instance="entry.visual"
      @edit="emit('edit', $event)"
    />
  </template>
  <Transition
    v-else
    :duration="widget.style.motionDuration"
    :enter-from-class="'motion-' + widget.style.showMotion"
    :leave-to-class="'motion-' + widget.style.hideMotion"
    enter-active-class="widget-motion-active"
    leave-active-class="widget-motion-active"
  >
    <div
      v-if="shown"
      class="scene-widget"
      :class="{
        'scene-widget-group': group,
        'is-selected': selected,
        'is-editable': editable,
        'is-disabled': editable && widget.enabled === false,
        'is-skill-history': historyGroup,
      }"
      :style="style"
      :data-widget-id="passive ? undefined : widget.id"
      :data-widget-movable="movable ? '' : undefined"
      :data-overlay-interactive="editable && !passive ? '' : undefined"
      @pointerdown.capture="captureMove"
      @pointerdown.stop="beginMove"
      @pointermove.stop="pointer?.move($event)"
      @pointerup.stop="pointer?.end($event)"
      @pointercancel.stop="pointer?.end($event)"
      @lostpointercapture.stop="pointer?.end($event)"
    >
      <span v-if="selected && !group" class="widget-selection-label">{{ widget.name }}</span>
      <div class="widget-content" :class="{ 'is-group': group }" :style="contentStyle">
        <span
          v-if="!effectGroup"
          class="widget-decoration"
          :style="{
            border:
              String(visual.style.borderWidth) +
              'px ' +
              visual.style.borderStyle +
              ' ' +
              visual.appearance.borderColor,
            boxSizing: 'border-box',
          }"
        />
        <IconWidget
          v-if="widget.kind === WidgetKind.Icon"
          :widget="visual"
          :runtime="runtime"
          :ticking="ticking"
          :sampled-at="sampledAt"
        />
        <TextWidget v-else-if="widget.kind === WidgetKind.Text" :widget="visual" />
        <BarWidget
          v-else-if="widget.kind === WidgetKind.Bar"
          :key="runtime?.progressIdentity ?? widget.id"
          :widget="visual"
          :runtime="runtime"
        />
        <template v-else>
          <WidgetGroupHeader
            v-if="widget.group.headerEnabled"
            :widget="widget"
            :session="session"
            :collapsed="collapsed"
            @collapse="collapsed = !collapsed"
            @edit="emit('edit', widget.id)"
          />
          <span v-else-if="editable" class="group-caption">{{ widget.name }}</span>
          <GroupEffectIcons
            v-if="effectGroup && !collapsed"
            :widget="visual"
            :runtime="runtime"
            :ticking="ticking"
            :sampled-at="sampledAt"
          />
          <WidgetView
            v-for="child in collapsed || effectGroup ? [] : widget.children"
            :key="child.id"
            :widget="child"
            :session="session"
            :parent-layout="layout"
            :parent="widget"
            :visuals="childVisuals"
            :selected-id="selectedId"
            :editable="editable"
            :scale="scale * factor"
            :pointer="pointer"
            :preview="preview"
            :passive="passive"
            :playback="playback"
            @edit="emit('edit', $event)"
          />
        </template>
      </div>
      <button
        v-if="selected"
        type="button"
        class="resize-handle"
        :aria-label="widget.name"
        :data-resize-id="widget.id"
        @pointerdown.stop="pointer?.begin($event, widget, scale, true, layout, true, factor)"
        @pointermove.stop="pointer?.move($event)"
        @pointerup.stop="pointer?.end($event)"
        @pointercancel.stop="pointer?.end($event)"
        @lostpointercapture.stop="pointer?.end($event)"
      />
    </div>
  </Transition>
</template>

<style scoped>
.is-skill-history:not(.is-editable) {
  contain: paint;
  isolation: isolate;
}

.widget-content {
  position: absolute;
  inset: 0 auto auto 0;
  display: flex;
  align-items: inherit;
  justify-content: inherit;
  transform-origin: top left;
}

.widget-content.is-group {
  display: block;
}
</style>
