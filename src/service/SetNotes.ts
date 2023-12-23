import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoFile } from '../entities/VideoFile';

export async function SetNotes(video: VideoFile, notes: string): Promise<ServiceResponse<string>> {
  return await invoke<IServiceResponse<string>>('set_video_notes', {
    file: video,
    notes
  }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}
