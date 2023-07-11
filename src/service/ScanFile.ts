import {ServiceResponse} from "./ServiceResponse";
import {FolderInfo} from "../entities/FolderInfo";
import {invoke} from "@tauri-apps/api";

export function ScanFile(path: string): Promise<ServiceResponse<FolderInfo>> {
    return invoke("file_scan", {path: path})
}