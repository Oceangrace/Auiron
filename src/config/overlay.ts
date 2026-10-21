import { CombatJournalModule } from '../modules/combat-journal/combat-journal-module';
import type { DesktopClient } from '../core/desktop/desktop-client';
import type { InterfaceEvents } from '../modules/interfaces/model/interface-events';
import { InterfacesModule } from '../modules/interfaces/interfaces-module';
import { TauriInterfaceStore } from '../modules/interfaces/persistence/tauri-interface-store';
import { CatalogClient } from '../modules/interfaces/triggers/catalog-client';
import type { CatalogLabels } from '../core/catalog/catalog-labels';

export class OverlayConfiguration {
  public journal(
    desktop: DesktopClient<InterfaceEvents>,
    locale: () => string,
    labels: () => CatalogLabels,
  ): CombatJournalModule {
    return new CombatJournalModule(desktop, locale, labels);
  }

  public create(
    desktop: DesktopClient<InterfaceEvents>,
    locale: () => string,
    labels: () => CatalogLabels,
  ): InterfacesModule {
    return new InterfacesModule(
      desktop,
      new TauriInterfaceStore(desktop),
      new CatalogClient(desktop, locale, labels),
      locale,
    );
  }
}
