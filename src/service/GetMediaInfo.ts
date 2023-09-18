import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { type VideoMediaInfo } from '../entities/VideoMediaInfo';

export async function GetMediaInfo(id: string, path: string): Promise<ServiceResponse<undefined>> {
  return await invoke<IServiceResponse<undefined>>('get_media_info', { id, path }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}

export interface VideoMediaInfoEmitEvent {
  media_info: VideoMediaInfo;
}
