import { type IServiceResponse, ServiceResponse } from './ServiceResponse';
import { invoke } from '@tauri-apps/api';

export async function OpenPath(path: string): Promise<ServiceResponse<boolean>> {
  return await invoke<IServiceResponse<boolean>>('open_path', { path }).then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    const success = response as boolean;
    return new ServiceResponse(result, success);
  });
}
