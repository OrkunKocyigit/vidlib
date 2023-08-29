import { type IVideoFile, VideoFile } from './VideoFile';
import { FileInfo, type IFileInfo } from './FileInfo';

export interface IFolderInfo extends IFileInfo {
  id: string;
  folders: IFolderInfo[];
  videos: IVideoFile[];
  empty: boolean;
  watched: boolean;
}

export class FolderInfo extends FileInfo implements IFolderInfo {
  empty: boolean;
  folders: FolderInfo[];
  videos: VideoFile[];
  id: string;
  watched: boolean;

  constructor(
    depth: number,
    name: string,
    path: string,
    empty: boolean,
    folders: IFolderInfo[],
    videos: IVideoFile[],
    id: string,
    watched: boolean
  ) {
    super(depth, name, path);
    this.empty = empty;
    this.folders = folders.map(
      ({ depth, name, path, empty, folders, videos, id, watched }) =>
        new FolderInfo(depth, name, path, empty, folders, videos, id, watched)
    );
    this.videos = videos.map(
      ({ id, depth, name, path, watched }) => new VideoFile(depth, name, path, id, watched)
    );
    this.id = id;
    this.watched = watched;
  }
}
