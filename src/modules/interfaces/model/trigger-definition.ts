import type { CatalogSelection } from './catalog-selection';
import type { ObservationKind } from './observation-kind';
import type { TriggerSubject } from './trigger-subject';

export interface TriggerDefinition {
  readonly subject: TriggerSubject;
  readonly observation: ObservationKind;
  readonly actorKey: string | null;
  readonly selection: CatalogSelection;
  readonly displaySeconds: number | null;
}
