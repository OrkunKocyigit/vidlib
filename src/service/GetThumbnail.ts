import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';

export async function GetThumbnail(
  id: string,
  path: string
): Promise<ServiceResponse<string[] | undefined>> {
  return await invoke<IServiceResponse<string[] | undefined>>('get_thumbnail', { id, path }).then(
    (value) => {
      const { error, result, response } = value;
      if (error !== null) {
        throw new Error(error);
      }
      const paths = response;
      if (paths === undefined || paths === null) {
        return new ServiceResponse(result, undefined);
      }
      const convertedPaths = [];
      for (const path of paths) {
        convertedPaths.push(convertFileSrc(path));
      }
      return new ServiceResponse(result, convertedPaths);
    }
  );
}

export interface GetThumbnailEvent {
  path: string;
}
