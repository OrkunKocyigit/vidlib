import { invoke } from '@tauri-apps/api';
import { type VideoFile } from '../entities/VideoFile';

export async function OpenVideo(video: VideoFile): Promise<unknown> {
  return await invoke('open_video', {
    video
  });
}
