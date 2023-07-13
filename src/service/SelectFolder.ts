import { invoke } from '@tauri-apps/api';
import { type IServiceResponse, ResponseType, ServiceResponse } from './ServiceResponse';
import { CancelError } from './CancelError';

export async function SelectFolder(): Promise<ServiceResponse<string>> {
  return await invoke<IServiceResponse<string>>('select_folder').then((value) => {
    const { error, result, response } = value;
    if (error !== null) {
      throw new Error(error);
    }
    if (result === ResponseType.CANCELED) {
      throw new CancelError();
    }
    return new ServiceResponse(result, response);
  });
}
