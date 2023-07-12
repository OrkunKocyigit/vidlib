import { type ServiceResponse } from './ServiceResponse';
import { type FolderInfo } from '../entities/FolderInfo';
import { invoke } from '@tauri-apps/api';

export async function ScanFile(path: string): Promise<ServiceResponse<FolderInfo>> {
  return await invoke('file_scan', { path });
}
