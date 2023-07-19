import { type VideoEntry } from '../entities/VideoEntry';
import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoFile } from '../entities/VideoFile';

export async function GetVideo(video: VideoFile): Promise<ServiceResponse<VideoEntry>> {
  return await invoke<IServiceResponse<VideoEntry>>('get_video', { video }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}
