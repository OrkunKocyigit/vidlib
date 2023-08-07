import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoFile } from '../entities/VideoFile';
import { type VideoMetadata } from '../entities/VideoMetadata';

export async function GetMetadata(video: VideoFile): Promise<ServiceResponse<VideoMetadata>> {
  return await invoke<IServiceResponse<VideoMetadata>>('get_metadata', { video }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}
