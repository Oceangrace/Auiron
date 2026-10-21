import type { DesktopCommands } from './desktop-commands';

export interface DesktopClient<TEvents> extends DesktopCommands {
  listen<TEvent extends keyof TEvents & string>(
    event: TEvent,
    receive: (payload: TEvents[TEvent]) => void,
  ): Promise<() => void>;
}
