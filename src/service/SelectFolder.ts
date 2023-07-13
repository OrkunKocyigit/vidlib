import { invoke } from '@tauri-apps/api';
import { type IServiceResponse, ServiceResponse } from './ServiceResponse';

export async function SelectFolder(): Promise<ServiceResponse<string>> {
  return await invoke<IServiceResponse<string>>('select_folder').then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    return new ServiceResponse(result, response);
  });
}
