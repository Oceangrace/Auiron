import type { InterfaceFormat } from './interface-format';
import type { InterfaceDocument } from './interface-document';

export interface InterfaceWorkspace {
  readonly formatVersion: InterfaceFormat.Current;
  readonly revision: number;
  readonly documents: readonly InterfaceDocument[];
}
