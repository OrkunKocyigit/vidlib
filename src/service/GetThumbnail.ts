import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoFile } from '../entities/VideoFile';

export async function GetThumbnail(video: VideoFile): Promise<ServiceResponse<string[]>> {
  return await invoke<IServiceResponse<string[]>>('get_thumbnail', { video }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}
