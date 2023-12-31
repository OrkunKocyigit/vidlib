import { FolderInfo, type IFolderInfo } from '../entities/FolderInfo';
import { invoke } from '@tauri-apps/api';
import { ServiceResponse, type IServiceResponse } from './ServiceResponse';

export async function AddFolder(path: string): Promise<ServiceResponse<FolderInfo>> {
  return await invoke<IServiceResponse<IFolderInfo>>('add_folder', { path }).then((value) => {
    const { error, result, response } = value;
    if (error !== null || response == null) {
      throw new Error(error);
    }
    const { depth, name, path, empty, folders, videos, id, watched } = response;
    return new ServiceResponse(
      result,
      new FolderInfo(depth, name, path, empty, folders, videos, id, watched)
    );
  });
}
