import {VideoFile} from "./VideoFile";

export interface FolderInfo {
    path: string,
    folders: FolderInfo[],
    videos: VideoFile[],
    empty: boolean,
    name: string
}