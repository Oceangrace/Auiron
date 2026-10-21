import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { DesktopClient } from '../../core/desktop/desktop-client';

export class TauriClient<TEvents> implements DesktopClient<TEvents> {
  public constructor(private readonly locale: () => string) {}

  public async invoke<T>(command: string, arguments_?: Record<string, unknown>): Promise<T> {
    return invoke<T>(command, { ...arguments_, locale: this.locale() });
  }

  public async listen<TEvent extends keyof TEvents & string>(
    event: TEvent,
    receive: (payload: TEvents[TEvent]) => void,
  ): Promise<() => void> {
    return listen<TEvents[TEvent]>(event, (message) => {
      receive(message.payload);
    });
  }
}
