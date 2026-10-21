export interface DesktopCommands {
  invoke<T>(command: string, arguments_?: Record<string, unknown>): Promise<T>;
}
