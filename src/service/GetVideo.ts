import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { VideoFile, type IVideoFile } from '../entities/VideoFile';

export async function GetVideo(video: VideoFile): Promise<ServiceResponse<VideoFile>> {
  return await invoke<IServiceResponse<IVideoFile>>('get_video', { video }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    const iVideo = response as IVideoFile;
    return new ServiceResponse(
      result,
      new VideoFile(iVideo.depth, iVideo.name, iVideo.path, iVideo.id, iVideo.watched, iVideo.video)
    );
  });
}
