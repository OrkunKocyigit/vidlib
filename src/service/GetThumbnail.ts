import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoFile } from '../entities/VideoFile';
import { convertFileSrc } from '@tauri-apps/api/tauri';

export async function GetThumbnail(video: VideoFile): Promise<ServiceResponse<string[]>> {
  return await invoke<IServiceResponse<string[]>>('get_thumbnail', { video }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    const paths = response as string[];
    const convertedPaths = [];
    for (const path of paths) {
      convertedPaths.push(convertFileSrc(path));
    }
    return new ServiceResponse(result, convertedPaths);
  });
}
