import { FolderInfo, type IFolderInfo } from '../entities/FolderInfo';
import { invoke } from '@tauri-apps/api';
import { type IServiceResponse, ServiceResponse } from './ServiceResponse';

export async function ScanFile(path: string): Promise<ServiceResponse<FolderInfo>> {
  return await invoke('file_scan', { path }).then((value) => {
    const { error, result, response } = value as IServiceResponse<IFolderInfo>;
    if (error !== null) {
      throw new Error(error);
    }
    const { depth, name, path, empty, folders, videos } = response as IFolderInfo;
    return new ServiceResponse<FolderInfo>(
      result,
      new FolderInfo(depth, name, path, empty, folders, videos)
    );
  });
}
