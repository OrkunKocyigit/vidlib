import { FolderInfo, type IFolderInfo } from '../entities/FolderInfo';
import { invoke } from '@tauri-apps/api';
import { ServiceResponse, type IServiceResponse } from './ServiceResponse';

export async function GetFolders(): Promise<ServiceResponse<FolderInfo[]>> {
  return await invoke<IServiceResponse<IFolderInfo[]>>('get_folders').then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    const folderInfos = [];
    for (const iFolderInfo of response as IFolderInfo[]) {
      const { depth, name, path, empty, folders, videos, id } = iFolderInfo;
      const folderInfo = new FolderInfo(depth, name, path, empty, folders, videos, id);
      folderInfos.push(folderInfo);
    }
    return new ServiceResponse(result, folderInfos);
  });
}
