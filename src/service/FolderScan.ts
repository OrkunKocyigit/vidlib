import { invoke } from '@tauri-apps/api';
import { type IServiceResponse, ServiceResponse } from './ServiceResponse';

export async function FolderScan(): Promise<ServiceResponse<undefined>> {
  return await invoke<IServiceResponse<undefined>>('folder_scan').then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}
