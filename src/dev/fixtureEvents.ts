/**
 * Stand-in for `@tauri-apps/api/event` under `KIT_UI_FIXTURES`. Nothing emits,
 * so listeners simply never fire — which is the honest behaviour outside Tauri.
 */

export type UnlistenFn = () => void;

export async function listen<T>(
  _event: string,
  _handler: (e: { payload: T }) => void
): Promise<UnlistenFn> {
  return () => {};
}

export async function emit(_event: string, _payload?: unknown): Promise<void> {}
