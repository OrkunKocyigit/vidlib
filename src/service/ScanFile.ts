import {ServiceResponse} from "./ServiceResponse";
import {FolderInfo} from "./FolderInfo";
import {VideoFile} from "./VideoFile";
import {invoke} from "@tauri-apps/api";

export interface ScanFileResult {
    path: String
    folders: FolderInfo
    videos: VideoFile
}

export function ScanFile(path: string): Promise<ServiceResponse<ScanFileResult>> {
    return invoke("file_scan", {path: path})
}