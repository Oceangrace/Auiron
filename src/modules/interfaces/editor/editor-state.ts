import type { InterfaceWorkspace } from '../model/interface-workspace';
import type { SaveStatus } from './save-status';

export interface EditorState {
  readonly workspace: InterfaceWorkspace;
  readonly documentId: string | null;
  readonly selectedId: string | null;
  readonly selectedIds: readonly string[];
  readonly saveStatus: SaveStatus;
  readonly message: string | null;
  readonly canUndo: boolean;
  readonly canRedo: boolean;
  readonly readOnly: boolean;
}
