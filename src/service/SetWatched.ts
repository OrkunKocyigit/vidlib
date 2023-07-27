import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoEntry } from '../entities/VideoEntry';

export async function SetWatched(
  video: VideoEntry,
  newWatched: boolean
): Promise<ServiceResponse<boolean>> {
  return await invoke<IServiceResponse<boolean>>('set_watched', {
    video,
    watched: newWatched
  }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response as boolean);
  });
}
