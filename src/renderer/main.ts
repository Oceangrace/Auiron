import './styles/widget-fonts.css';
import { computed, createApp } from 'vue';
import { IconAssets } from './assets/icon-assets';
import { iconAssetsContextKey } from './assets/icon-assets-context-key';
import { createI18n } from 'vue-i18n';
import { OverlayConfiguration } from '../config/overlay';
import { TauriClient } from '../integrations/tauri/tauri-client';
import type { InterfaceEvents } from '../modules/interfaces/model/interface-events';
import type { LocalizationSnapshot } from '../core/localization/localization-snapshot';
import type { CatalogLabels } from '../core/catalog/catalog-labels';
import { combatJournalContextKey } from '../modules/combat-journal/combat-journal-context-key';
import { interfacesContextKey } from '../modules/interfaces/interfaces-context-key';
import { localizationContextKey } from './localization/localization-context-key';
import App from './app.vue';
import './overlay.css';
import './styles/theme.css';
import './styles/select.css';

document.addEventListener('contextmenu', (event) => {
  event.preventDefault();
});

document.documentElement.dataset['surface'] =
  new URLSearchParams(window.location.search).get('surface') ?? 'overlay';

const desktop = new TauriClient<InterfaceEvents>(() => document.documentElement.lang);

const start = async (): Promise<void> => {
  const assets = new IconAssets();

  const stopIcons = await desktop.listen('icons:changed', (files) => {
    assets.add(files);
  });

  // Subscribe before reading the cache so a completed download cannot be missed.
  assets.add(await desktop.invoke<readonly string[]>('icon_files'));
  const [resources, catalogs] = await Promise.all([
    desktop.invoke<LocalizationSnapshot>('localization_read'),
    desktop.invoke<Readonly<Record<string, CatalogLabels>>>('catalog_labels'),
  ]);

  const fallback = catalogs[resources.fallbackLocale];

  if (fallback === undefined) {
    throw new Error('Fallback catalog labels are missing');
  }

  const i18n = createI18n({
    legacy: false,
    locale: resources.defaultLocale,
    fallbackLocale: resources.fallbackLocale,
    messages: resources.messages,
  });

  const availableLabels = computed((): CatalogLabels => {
    const catalog = catalogs[i18n.global.locale.value] ?? fallback;

    return { ...catalog, icons: assets.labels(catalog.icons) };
  });

  const labels = (): CatalogLabels => availableLabels.value;

  const interfaces = new OverlayConfiguration().create(
    desktop,
    () => i18n.global.locale.value,
    labels,
  );

  const journal = new OverlayConfiguration().journal(
    desktop,
    () => i18n.global.locale.value,
    labels,
  );

  createApp(App)
    .use(i18n)
    .provide(iconAssetsContextKey, assets)
    .provide(interfacesContextKey, interfaces)
    .provide(combatJournalContextKey, journal)
    .provide(localizationContextKey, resources)
    .mount('#app');
  await Promise.all([interfaces.start(), journal.start()]);
  window.addEventListener('beforeunload', () => {
    stopIcons();
    interfaces.dispose();
    journal.dispose();
  });
};

void start().catch((error: unknown) => {
  const root = document.getElementById('app');

  if (root !== null) {
    root.textContent = String(error);
  }
});
