import { AnchorMode } from './anchor-mode';

export interface WidgetAnchors {
  readonly horizontal: AnchorMode;
  readonly vertical: AnchorMode;
  readonly right: number;
  readonly bottom: number;
  readonly widthPercent: number | null;
  readonly heightPercent: number | null;
}

export const defaultWidgetAnchors: WidgetAnchors = {
  horizontal: AnchorMode.Start,
  vertical: AnchorMode.Start,
  right: 0,
  bottom: 0,
  widthPercent: null,
  heightPercent: null,
};
