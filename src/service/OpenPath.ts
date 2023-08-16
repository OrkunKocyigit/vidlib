import { invoke } from '@tauri-apps/api';

export async function OpenPath(path: string): Promise<unknown> {
  return await invoke('open_path', {
    path
  });
}
