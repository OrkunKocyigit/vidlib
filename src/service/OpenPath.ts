import { invoke } from '@tauri-apps/api';

export async function OpenPath(path: string, parent = false): Promise<unknown> {
  return await invoke('open_path', {
    path,
    parent
  });
}
