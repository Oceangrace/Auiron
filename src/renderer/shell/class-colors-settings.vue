<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { ShellController } from './shell-controller';
import { CharacterClass } from '../../core/preferences/character-class';
import { ClassPalette } from '../../core/preferences/class-palette';
import UiColor from '../components/ui/ui-color.vue';
import UiButton from '../components/ui/ui-button.vue';

const props = defineProps<{ shell: ShellController }>();

const { t } = useI18n();

const preferences = props.shell.getState();

const palette = new ClassPalette();

const classes = Object.values(CharacterClass);

const update = (name: CharacterClass, color: string): void => {
  props.shell.update({
    classColors: { ...preferences.value.classColors, [name]: color.slice(0, 7) },
  });
};
</script>
<template>
  <section class="class-colors-settings">
    <h3>{{ t('shell.classColors.title') }}</h3>
    <div class="class-color-grid">
      <label v-for="name in classes" :key="name" class="class-color-field">
        <span>{{ t('modules.interfaces.classes.' + name) }}</span>
        <UiColor
          :model-value="preferences.classColors[name] ?? palette.defaults[name]"
          :label="t('modules.interfaces.classes.' + name)"
          @update:model-value="update(name, $event)"
        />
      </label>
    </div>
    <UiButton @click="shell.update({ classColors: {} })">{{
      t('shell.classColors.reset')
    }}</UiButton>
  </section>
</template>
<style scoped>
.class-colors-settings {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding-top: 24px;
  border-top: 1px solid var(--prism-panel-border);
}
.class-colors-settings h3 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
}
.class-color-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}
.class-color-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-width: 0;
}
.class-color-field > span {
  font-size: 12px;
  color: var(--prism-label);
}
.class-colors-settings > .ui-button {
  align-self: flex-start;
}
@media (max-width: 400px) {
  .class-color-grid {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
