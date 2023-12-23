import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoFile } from '../entities/VideoFile';

export async function SetRating(
  video: VideoFile,
  newRating: number
): Promise<ServiceResponse<number>> {
  return await invoke<IServiceResponse<number>>('set_video_rating', {
    file: video,
    rating: newRating
  }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}
