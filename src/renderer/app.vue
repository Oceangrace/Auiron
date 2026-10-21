<script setup lang="ts">
import { combatJournalContextKey } from '../modules/combat-journal/combat-journal-context-key';
import JournalSurface from '../modules/combat-journal/renderer/journal-surface.vue';

import UiMessage from './components/ui/ui-message.vue';
import { computed, inject, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { interfacesContextKey } from '../modules/interfaces/interfaces-context-key';
import InterfacesPanel from '../modules/interfaces/editor/interfaces-panel.vue';
import RuntimeStatus from '../modules/interfaces/editor/runtime-status.vue';
import OverlaySurface from '../modules/interfaces/scene/overlay-surface.vue';
import { EditorShortcuts } from '../modules/interfaces/editor/editor-shortcuts';
import { TauriClient } from '../integrations/tauri/tauri-client';
import type { InterfaceEvents } from '../modules/interfaces/model/interface-events';
import { ShellController } from './shell/shell-controller';
import { SurfaceController } from './shell/surface-controller';
import FloatingSurface from './shell/floating-surface.vue';
import ApplicationToolbar from './shell/application-toolbar.vue';
import { localizationContextKey } from './localization/localization-context-key';
import ApplicationSettings from './shell/application-settings.vue';
import WelcomeScreen from './shell/welcome-screen.vue';
import { StartupWelcome } from './shell/startup-welcome';
import { WelcomeStage } from './shell/welcome-stage';
import TrayNotice from './shell/tray-notice.vue';
const interfaces = inject(interfacesContextKey);

if (interfaces === undefined) {
  throw new Error('Interfaces module is missing');
}

const journal = inject(combatJournalContextKey);

if (journal === undefined) {
  throw new Error('Combat journal module is missing');
}

const ready = interfaces.getReady();

const state = interfaces.session.getState();

const editing = interfaces.getEditing();

const hudEnabled = interfaces.getHudEnabled();

const foreground = interfaces.getForeground();

const welcome = new StartupWelcome();

const welcomeStage = welcome.state();

const welcomeGuide = welcome.guideRequested();

watch(
  interfaces.getGameAttached(),
  (attached) => {
    welcome.observe(attached);
  },
  { immediate: true },
);

const previewEnabled = interfaces.preview.enabled;

const liveFrame = interfaces.live.getFrame();

const liveError = interfaces.live.getError();

const { t, locale } = useI18n();

const desktop = new TauriClient<InterfaceEvents>(() => locale.value);

const openEditor = (): void => {
  welcome.dismiss();
  interfaces.open();
};

const hideWelcome = (): void => {
  welcome.collapse();
};

const finishNotice = (): void => {
  if (welcomeStage.value !== WelcomeStage.Notice) {
    return;
  }

  welcome.dismiss();
  void desktop.invoke('app_wait_for_game').catch((error: unknown) => {
    interfaces.session.report(error);
  });
};

const resources = inject(localizationContextKey);

if (resources === undefined) {
  throw new Error('Localization resources are missing');
}

const shell = new ShellController(desktop, resources.defaultLocale);

const preferences = shell.getState();

const settingsOpen = ref(false);

watch(
  editing,
  (open, previous) => {
    if (open === true) {
      welcome.dismiss();

      return;
    }

    if (previous === true && settingsOpen.value === false && previewEnabled.value === false) {
      welcome.collapse();
    }
  },
  { immediate: true },
);

const settingsLayout = computed(
  () =>
    preferences.value.settingsPanel ?? {
      x: Math.max(24, window.innerWidth - 414),
      y: 80,
      width: 390,
      height: 570,
    },
);

const placementRevision = ref(0);

const root = ref<HTMLElement | null>(null);

let surface: SurfaceController | null = null;

let quitSubscription: (() => void) | null = null;

const shortcuts = new EditorShortcuts((action) => {
  interfaces.execute(action);
});

const key = (event: KeyboardEvent): void => {
  if (editing.value === true) {
    shortcuts.handle(event);
  }
};

const applyAppearance = (): void => {
  locale.value =
    resources.messages[preferences.value.locale] === undefined
      ? resources.fallbackLocale
      : preferences.value.locale;
  document.documentElement.lang = locale.value;
  document.title = t('modules.interfaces.title');
  shell.apply(document.documentElement, window.innerHeight);
};

watch(preferences, applyAppearance);

const quit = async (): Promise<void> => {
  await shell.flush();
  await journal.flush();
  await interfaces.quit();
};

onMounted(() => {
  void desktop
    .listen('app:quit-requested', () => {
      void quit();
    })
    .then((unsubscribe) => {
      quitSubscription = unsubscribe;
    })
    .catch((error: unknown) => {
      interfaces.session.report(error);
    });
  void shell.start().then(applyAppearance);

  if (root.value !== null) {
    surface = new SurfaceController(root.value, desktop, (error) => {
      interfaces.session.report(error);
    });
    surface.start();
  }

  window.addEventListener('keydown', key);
  window.addEventListener('resize', applyAppearance);
});
onBeforeUnmount(() => {
  surface?.dispose();
  quitSubscription?.();
  window.removeEventListener('keydown', key);
  window.removeEventListener('resize', applyAppearance);
});
</script>
<template>
  <main
    v-show="
      welcomeStage !== WelcomeStage.Hidden ||
      editing ||
      settingsOpen ||
      previewEnabled ||
      foreground
    "
    ref="root"
    class="application-surface"
  >
    <template v-if="ready">
      <WelcomeScreen
        v-if="welcomeStage === WelcomeStage.Window"
        :error="state.message ?? liveError"
        :show-guide="welcomeGuide"
        @editor="openEditor"
        @dismiss="hideWelcome"
      />
      <TrayNotice
        v-else-if="welcomeStage === WelcomeStage.Notice"
        @editor="openEditor"
        @guide="welcome.openGuide()"
        @dismiss="finishNotice"
      />
      <template v-else>
        <OverlaySurface :interfaces="interfaces" />
        <ApplicationToolbar
          :shell="shell"
          :editor-open="editing"
          :settings-open="settingsOpen"
          @editor="editing ? void interfaces.close() : interfaces.open()"
          @settings="settingsOpen = !settingsOpen"
          @quit="void quit()"
        >
          <template #status>
            <RuntimeStatus :frame="liveFrame" :error="liveError" />
          </template>
        </ApplicationToolbar>
        <InterfacesPanel
          v-if="editing"
          :key="'editor' + String(placementRevision)"
          :interfaces="interfaces"
          @close="void interfaces.close()"
        />
        <JournalSurface :journal="journal" :meters-visible="interfaces.gameSurfaceVisible()" />
        <FloatingSurface
          v-if="settingsOpen"
          :key="'settings' + String(placementRevision)"
          :title="t('shell.settings')"
          :x="settingsLayout.x"
          :y="settingsLayout.y"
          :width="settingsLayout.width"
          :height="settingsLayout.height"
          :min-width="390"
          :min-height="260"
          @close="settingsOpen = false"
          @moved="shell.saveSettingsPanel({ ...settingsLayout, ...$event })"
          @resized="shell.saveSettingsPanel($event)"
          ><ApplicationSettings
            :shell="shell"
            :hud-enabled="hudEnabled"
            :journal-enabled="journal.isVisible()"
            @hud="interfaces.toggleHud()"
            @journal="journal.toggle()"
            @reset="placementRevision++"
        /></FloatingSurface>
      </template>
    </template>
    <p v-else class="startup-message">
      <UiMessage :text="state.message ?? t('modules.interfaces.loading')" />
    </p>
  </main>
</template>
