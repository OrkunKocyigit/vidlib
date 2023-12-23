import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';

export async function DeletePath(path: string): Promise<ServiceResponse<boolean>> {
  return await invoke<IServiceResponse<boolean>>('delete_path', { path }).then((value) => {
    const { error, result, response } = value;
    if (error !== null || response == null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}
