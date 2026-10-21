import type { CatalogSelection } from '../model/catalog-selection';
import type { CatalogScope } from '../model/catalog-scope';
import type { TriggerSubject } from '../model/trigger-subject';
import type { ObservationKind } from '../model/observation-kind';

export interface TriggerDraft {
  readonly subject: TriggerSubject;
  readonly observation: ObservationKind;
  readonly actorKey: string | null;
  readonly contextKey: string | null;
  readonly scope: CatalogScope;
  readonly selection: CatalogSelection | null;
  readonly seconds: number;
  readonly iconPath: string | null;
}
