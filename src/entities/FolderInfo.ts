import { type IVideoFile, VideoFile } from './VideoFile';
import { FileInfo, type IFileInfo } from './FileInfo';

export interface IFolderInfo extends IFileInfo {
  id: string;
  folders: IFolderInfo[];
  videos: IVideoFile[];
  empty: boolean;
}

export class FolderInfo extends FileInfo implements IFolderInfo {
  empty: boolean;
  folders: FolderInfo[];
  videos: VideoFile[];
  id: string;

  constructor(
    depth: number,
    name: string,
    path: string,
    empty: boolean,
    folders: IFolderInfo[],
    videos: IVideoFile[],
    id: string
  ) {
    super(depth, name, path);
    this.empty = empty;
    this.folders = folders.map(
      ({ depth, name, path, empty, folders, videos, id }) =>
        new FolderInfo(depth, name, path, empty, folders, videos, id)
    );
    this.videos = videos.map(
      ({ id, depth, name, path, watched }) => new VideoFile(depth, name, path, id, watched)
    );
    this.id = id;
  }
}
