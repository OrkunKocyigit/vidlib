import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoEntry } from '../entities/VideoEntry';

export async function SetRating(
  video: VideoEntry,
  newRating: number
): Promise<ServiceResponse<number>> {
  return await invoke<IServiceResponse<number>>('set_video_rating', {
    video,
    rating: newRating
  }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response as number);
  });
}
