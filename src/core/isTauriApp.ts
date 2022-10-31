export function isTauriApp() {
  return Boolean(typeof window !== 'undefined' && window.__TAURI_IPC__);
}
